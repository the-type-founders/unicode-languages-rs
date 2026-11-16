# /// script
# dependencies = ["language-tags", "pyyaml"]
# ///

import json
import os
import sys

import yaml
from language_tags import tags

ROOT = os.path.dirname(os.path.abspath(__file__))


class Loader(yaml.SafeLoader):
    pass


def construct_range(loader, node):
    lower, upper = loader.construct_scalar(node).split("..")
    return [int(lower), int(upper)]


Loader.add_constructor("!ruby/range", construct_range)

data = []
for tag in sorted(os.listdir(os.path.join(ROOT, "source", "data"))):
    # Skip the files that are not named after a language, such as those named after a script.
    if not tags.check(tag):
        continue
    with open(os.path.join(ROOT, "source", "data", tag)) as file:
        # Some files use tabs in front of comments, which are not allowed in YAML.
        language = yaml.load(file.read().replace("\t", " "), Loader=Loader)
    codepoints = [[value, value] if isinstance(value, int) else value for value in language["codepoints"]]
    data.append(
        {
            "tag": tag,
            "name": language["anglicized_name"],
            "native": language["native_name"],
            "codepoints": sorted(codepoints),
        }
    )
json.dump(data, sys.stdout, ensure_ascii=False, indent=2)
sys.stdout.write("\n")
