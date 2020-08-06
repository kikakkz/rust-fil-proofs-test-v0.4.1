import sys


def ts_from_line(line):
    strs = line.split(" ")
    if 0 < len(strs):
        time_strs = strs[0].split("T")
        if 2 <= len(time_strs):
            hms = time_strs[1].split(":")
            h = int(hms[0])
            m = int(hms[1])
            mss = hms[2].split(".")
            s = int(mss[0])
            ms = int(mss[1])
            return h * 60 * 60 * 1000 + m * 60 * 1000 + s * 1000 + ms
    return -1


def main():
    layer_start = 0
    layer0_end = 0
    layer1_end = 0
    layer2_end = 0
    layer3_end = 0
    layer4_end = 0
    layer5_end = 0
    layer6_end = 0
    layer7_end = 0
    layer8_end = 0
    layer9_end = 0
    layer10_end = 0

    rounds = 0

    layer0_total = 0
    layer1_total = 0
    layer2_total = 0
    layer3_total = 0
    layer4_total = 0
    layer5_total = 0
    layer6_total = 0
    layer7_total = 0
    layer8_total = 0
    layer9_total = 0
    layer10_total = 0

    layer_total = 0

    layer0_max = 0
    layer1_max = 0
    layer2_max = 0
    layer3_max = 0
    layer4_max = 0
    layer5_max = 0
    layer6_max = 0
    layer7_max = 0
    layer8_max = 0
    layer9_max = 0
    layer10_max = 0

    reorg_total = 0
    reorg_start = 0
    reorg_end = 0

    gpu_idle_start = 0
    gpu_idle_end = 0
    gpu_idle_total = 0

    for line in open("./p2.log"):
        if 'start to reorganize' in line or 'START reorganize column' in line:
            reorg_start = ts_from_line(line)
        elif 'node index' in line:
            reorg_end = ts_from_line(line)
            reorg_total += reorg_end - reorg_start
        elif 'waiting for next column' in line:
            gpu_idle_start = ts_from_line(line)
        elif 'next columns received' in line or 'next column received' in line:
            gpu_idle_end = ts_from_line(line)
            gpu_idle_total += gpu_idle_end - gpu_idle_start
        elif 'start to fill' in line or 'START current layer' in line:
            layer_start = ts_from_line(line)
            rounds += 1
        elif 'current layer 0' in line:
            layer0_end = ts_from_line(line)
            layer0_total += layer0_end - layer_start
            layer0_max = max(layer0_max, layer0_end - layer_start)
        elif 'current layer 10' in line:
            layer10_end = ts_from_line(line)
            layer10_total += layer10_end - layer9_end
            layer10_max = max(layer10_max, layer10_end - layer9_end)
            layer_total += layer10_end - layer_start
        elif 'current layer 1' in line:
            layer1_end = ts_from_line(line)
            layer1_total += layer1_end - layer0_end
            layer1_max = max(layer1_max, layer1_end - layer0_end)
        elif 'current layer 2' in line:
            layer2_end = ts_from_line(line)
            layer2_total += layer2_end - layer1_end
            layer2_max = max(layer2_max, layer2_end - layer1_end)
        elif 'current layer 3' in line:
            layer3_end = ts_from_line(line)
            layer3_total += layer3_end - layer2_end
            layer3_max = max(layer3_max, layer3_end - layer2_end)
        elif 'current layer 4' in line:
            layer4_end = ts_from_line(line)
            layer4_total += layer4_end - layer3_end
            layer4_max = max(layer4_max, layer4_end - layer3_end)
        elif 'current layer 5' in line:
            layer5_end = ts_from_line(line)
            layer5_total += layer5_end - layer4_end
            layer5_max = max(layer5_max, layer5_end - layer4_end)
        elif 'current layer 6' in line:
            layer6_end = ts_from_line(line)
            layer6_total += layer6_end - layer5_end
            layer6_max = max(layer6_max, layer6_end - layer5_end)
        elif 'current layer 7' in line:
            layer7_end = ts_from_line(line)
            layer7_total += layer7_end - layer6_end
            layer7_max = max(layer7_max, layer7_end - layer6_end)
        elif 'current layer 8' in line:
            layer8_end = ts_from_line(line)
            layer8_total += layer8_end - layer7_end
            layer8_max = max(layer8_max, layer8_end - layer7_end)
        elif 'current layer 9' in line:
            layer9_end = ts_from_line(line)
            layer9_total += layer9_end - layer8_end
            layer9_max = max(layer9_max, layer9_end - layer8_end)
        elif 'tree_c done' in line:
            break    

    print("Layer all total -> {} / avg -> {} / rounds -> {}" . format(layer_total, layer_total / rounds, rounds))
    print("Layer 0 total -> {} / avg -> {} / max -> {}" . format(layer0_total, layer0_total / rounds, layer0_max))
    print("Layer 1 total -> {} / avg -> {} / max -> {}" . format(layer1_total, layer1_total / rounds, layer1_max))
    print("Layer 2 total -> {} / avg -> {} / max -> {}" . format(layer2_total, layer2_total / rounds, layer2_max))
    print("Layer 3 total -> {} / avg -> {} / max -> {}" . format(layer3_total, layer3_total / rounds, layer3_max))
    print("Layer 4 total -> {} / avg -> {} / max -> {}" . format(layer4_total, layer4_total / rounds, layer4_max))
    print("Layer 5 total -> {} / avg -> {} / max -> {}" . format(layer5_total, layer5_total / rounds, layer5_max))
    print("Layer 6 total -> {} / avg -> {} / max -> {}" . format(layer6_total, layer6_total / rounds, layer6_max))
    print("Layer 7 total -> {} / avg -> {} / max -> {}" . format(layer7_total, layer7_total / rounds, layer7_max))
    print("Layer 8 total -> {} / avg -> {} / max -> {}" . format(layer8_total, layer8_total / rounds, layer8_max))
    print("Layer 9 total -> {} / avg -> {} / max -> {}" . format(layer9_total, layer9_total / rounds, layer9_max))
    print("Layer 10 total -> {} / avg -> {} / max -> {}" . format(layer10_total, layer10_total / rounds, layer10_max))
    print("Reorg total -> {} / avg -> {}" . format(reorg_total, reorg_total / rounds))
    print("GPU idle total -> {} / avg -> {}" . format(gpu_idle_total, gpu_idle_total / rounds))


if __name__ == "__main__":
    main()
