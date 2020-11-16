from midas import (
    MidasR,
    hash,
)
import csv

if __name__ == "__main__":
    # alpha is the decay value.
    #
    # With every increment in the time value, all the stored bucket
    # values are multiplied by this number.
    #
    # The alpha value below means that a value of 1.0 at t_0 will decay
    # to 0.8 by t_30
    alpha = 0.8 ** (1.0 / 30.0)
    midas = MidasR(alpha=alpha)
    with open("in.csv") as infile, open("test.out.csv", "w") as outfile:
        reader = csv.reader(infile)
        writer = csv.writer(outfile)
        for line in reader:
            source = hash(line[0])
            dest = hash(line[1])
            time = int(line[2])
            writer.writerow([
                line[0],
                line[1],
                time,
                "{:.6f}".format(midas.insert(source, dest, time)),
            ])
