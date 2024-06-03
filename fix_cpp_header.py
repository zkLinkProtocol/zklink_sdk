import sys

L1SignerTypeStart = 0
with open(sys.argv[1]) as reader:
    lines = reader.readlines()
    for (num, line) in enumerate(lines):
        if line == "    struct FfiConverterSigner;\n":
            SignerStart = num - 2
            continue
        if line == "struct FfiConverterTypeL1SignerType;\n":
            L1SignerTypeStart = num - 2
            continue
        if L1SignerTypeStart > 0 and line == "};\n":
            L1SignerTypeEnd = num + 1
            break

with open(sys.argv[1], 'w') as writer:
    writer.writelines(lines[:SignerStart])
    writer.writelines(lines[L1SignerTypeStart:L1SignerTypeEnd])
    writer.writelines(lines[SignerStart:L1SignerTypeStart])
    writer.writelines(lines[L1SignerTypeEnd:])
