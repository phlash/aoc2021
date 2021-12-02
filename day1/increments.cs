class increments {
    public static void Main(string[] args) {
        int[] win = new int[3]{0,0,0};
        int prv = 0;
        int prw = 0;
        int cnv = 0;
        int cnw = 0;
        var lines = System.IO.File.ReadLines("input");
        int idx = 0;
        foreach (var line in lines) {
            int val = System.Convert.ToInt32(line);
            // populate the window
            win[idx%3] = val;
            // compare (if beyond first value)
            if (idx>0) {
                if (val>prv) {
                    cnv = cnv+1;
                }
            }
            // calculate sum
            int sum = win[0]+win[1]+win[2];
            // compare window (if beyond first three values)
            if (idx>2) {
                if (sum>prw) {
                    cnw = cnw+1;
                }
            }
            prv = val;
            prw = sum;
            idx = idx+1;
        }
        System.Console.WriteLine($"Increments: single:{cnv} windowed:{cnw}");
    }
}
