# /// script
# requires-python = ">=3.9"
# ///

import json
import re
import sys
import xml.etree.ElementTree as ET
from io import BytesIO
from urllib.request import urlopen
from zipfile import ZipFile

VERSION = "49"
ROOT = f"https://www.unicode.org/Public/cldr/{VERSION}"

# The first archive contains the locales at the basic level of coverage or
# above, and the second contains the rest.
ARCHIVES = ["core.zip", f"cldr-exemplars-delta-{VERSION}.zip"]

MAIN = re.compile(r"common/main/(\w+)\.xml")
LIKELY = "common/supplemental/likelySubtags.xml"
NAMES = "localeDisplayNames/languages/language"
SETS = "characters/exemplarCharacters"

# The number of hexadecimal digits following an escape character.
ESCAPES = {"u": 4, "U": 8}


def read(name):
    with urlopen(f"{ROOT}/{name}") as response:
        return ZipFile(BytesIO(response.read()))


def parse(value):
    value = value.strip()
    if not (value.startswith("[") and value.endswith("]")):
        raise ValueError(f"expected a set: {value}")
    value = value[1:-1]
    codepoints = set()
    sequence = False
    index = 0
    while index < len(value):
        character = value[index]
        index += 1
        if character.isspace():
            continue
        # A string in braces is a sequence, and each of its characters counts.
        if character == "{" and not sequence:
            sequence = True
        elif character == "}" and sequence:
            sequence = False
        elif character == "\\":
            character = value[index]
            index += 1
            if character in ESCAPES:
                size = ESCAPES[character]
                codepoints.add(int(value[index : index + size], 16))
                index += size
            else:
                codepoints.add(ord(character))
        # The rest of the syntax, such as ranges and nested sets, is not used.
        elif character in "[]{}-&^$":
            raise ValueError(f"found an unsupported construct: {value}")
        else:
            codepoints.add(ord(character))
    if sequence:
        raise ValueError(f"found an unterminated sequence: {value}")
    return codepoints


def merge(codepoints):
    ranges = []
    for codepoint in sorted(codepoints):
        # The upper bound is exclusive.
        if ranges and ranges[-1][1] == codepoint:
            ranges[-1][1] = codepoint + 1
        else:
            ranges.append([codepoint, codepoint + 1])
    return ranges


def split(locale):
    language, *rest = locale.split("_")
    script = next((part for part in rest if re.fullmatch("[A-Z][a-z]{3}", part)), None)
    region = next(
        (part for part in rest if re.fullmatch(r"[A-Z]{2}|\d{3}", part)), None
    )
    variant = next((part for part in rest if part not in (script, region)), None)
    return language, script, region, variant


roots = {}
likely = {}
for archive in map(read, ARCHIVES):
    for path in archive.namelist():
        if match := MAIN.fullmatch(path):
            roots[match[1]] = ET.fromstring(archive.read(path))
        elif path == LIKELY:
            for element in ET.fromstring(archive.read(path)).iter("likelySubtag"):
                likely[element.get("from")] = element.get("to")

names = {
    element.get("type"): element.text
    for element in roots["en"].findall(NAMES)
    if element.get("alt") is None
}

data = []
for locale, root in sorted(roots.items()):
    # The main set, as opposed to the auxiliary one and such, has no type.
    sets = [
        element.text for element in root.findall(SETS) if element.get("type") is None
    ]
    codepoints = set().union(*map(parse, sets))
    # A locale without a set of its own inherits one and adds nothing.
    if not codepoints:
        continue
    language, script, region, variant = split(locale)
    variant = variant and variant.lower()
    code = "-".join(filter(None, (language, script, region, variant)))
    # The script is implied when it is the most likely one.
    if not script:
        candidates = [f"{language}_{region}", language] if region else [language]
        script = split(next(likely[key] for key in candidates if key in likely))[1]
    data.append(
        {
            "code": code,
            # The name of the language alone is used when the locale has none.
            "name": names.get(locale) or names.get(language),
            "script": script,
            "ranges": merge(codepoints),
        }
    )
json.dump(data, sys.stdout, ensure_ascii=False, indent=2)
sys.stdout.write("\n")
