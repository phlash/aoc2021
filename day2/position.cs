class position {
    public static void Main() {
        int hoz = 0;
        int de1 = 0;
        int de2 = 0;
        int aim = 0;
        var lines = System.IO.File.ReadLines("input");
        foreach (var line in lines) {
            // each line has '<command> <distance>'
            string[] vals = line.Split();
            string cmd = vals[0];
            int dis = System.Convert.ToInt32(vals[1]);
            switch(cmd) {
            case "forward": hoz = hoz+dis; de2 = de2+(aim*dis); break;
            case "down": de1 = de1+dis; aim = aim+dis; break;
            case "up": de1 = de1-dis; aim = aim-dis; break;
            }
        }
        System.Console.WriteLine($"hoz:{hoz} de1:{de1} de2:{de2} mu1:{hoz*de1} mu2:{hoz*de2}");
    }
}
