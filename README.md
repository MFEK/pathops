# MFEKpathops

(c) 2021 Fredrick R. Brennan & MFEK Authors

A utility for applying path operations to contours (in UFO .glif format).

```
MFEKpathops-BOOLEAN 0.1.0

Fredrick Brennan <copypasteⒶkittens.ph>; Skia Authors; Andrew Hunter (flo_curves.rs); MFEK Authors

Applies a boolean (union/intersect/difference/XOR…) operation to a glyph in UFO .glif format. Some
of the algorithms use Skia, others use flo_curves.

USAGE:
    MFEKpathops BOOLEAN [OPTIONS] --input <input> --output <output>

OPTIONS:
    -p, --pathop <pathop>      Boolean operation to apply. [skia values: difference, intersect,
                               union, xor, reverse_difference] [flo_curves values: add,
                               flo_intersect, remove_interior, remove_overlapping, sub] [default:
                               union]
    -i, --input <input>        The path to the input glif file.
    -O, --operand <operand>    The path to the glif file that will act as the operand to the boolean
                               operation. (skia: required if <pathop> not union.)  (flo_curves: only
                               used if mode is flo_intersect, remove_interior or remove_overlapping)
    -o, --output <output>      The path to the output glif file.
    -h, --help                 Print help information
    -V, --version              Print version information
```

```
MFEKpathops-CLEAR 0.0.0

Fredrick Brennan <copypasteⒶkittens.ph>; MFEK Authors

Delete all contours in glyph

USAGE:
    MFEKpathops CLEAR [OPTIONS] --input <input>

OPTIONS:
    -i, --input <input>        The path to the input UFO `.glif` file. (will be overwritten!)
    -P, --prune-contour-ops    Prune contour ops?
    -h, --help                 Print help information
    -V, --version              Print version information
```

```
MFEKpathops-FIT 0.1.0

T Prajwal Prabhu <prajwalprabhu.tellar@gmail.com>

Returns control points of an cubic bezier curve accorfing to knot(end) points

USAGE:
    MFEKpathops FIT --input <input> --output <output>

OPTIONS:
    -i, --input <input>      The path to the input glif file.
    -o, --output <output>    The path to the output glif file.
    -h, --help               Print help information
    -V, --version            Print version information
```

## License

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at:

http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.


