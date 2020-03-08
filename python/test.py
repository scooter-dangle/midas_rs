from midas import (
    MidasR,
    hash,
)
import csv

if __name__ == "__main__":
    midas = MidasR()
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
