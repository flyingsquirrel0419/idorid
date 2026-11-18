package java.util;
public class Random implements java.io.Serializable {
    private static final long multiplier=0x5deece66dL, addend=0xbL, mask=(1L<<48)-1;
    private long seed; private double nextNextGaussian; private boolean haveNextNextGaussian;
    public Random(){this(System.nanoTime());} public Random(long seed){setSeed(seed);}
    public synchronized void setSeed(long seed){this.seed=(seed^multiplier)&mask;haveNextNextGaussian=false;}
    protected synchronized int next(int bits){seed=(seed*multiplier+addend)&mask;return (int)(seed>>>(48-bits));}
    public void nextBytes(byte[] bytes){for(int i=0;i<bytes.length;){int rnd=nextInt(),n=Math.min(bytes.length-i,4);while(n-->0){bytes[i++]=(byte)rnd;rnd>>=8;}}}
    public int nextInt(){return next(32);}
    public int nextInt(int bound){if(bound<=0)throw new IllegalArgumentException("bound must be positive");int r=next(31),m=bound-1;if((bound&m)==0)r=(int)((bound*(long)r)>>31);else{int u=r;while(u-(r=u%bound)+m<0)u=next(31);}return r;}
    public long nextLong(){return ((long)next(32)<<32)+next(32);}
    public boolean nextBoolean(){return next(1)!=0;}
    public float nextFloat(){return next(24)/((float)(1<<24));}
    public double nextDouble(){return (((long)next(26)<<27)+next(27))/(double)(1L<<53);}
    public synchronized double nextGaussian(){if(haveNextNextGaussian){haveNextNextGaussian=false;return nextNextGaussian;}double v1,v2,s;do{v1=2*nextDouble()-1;v2=2*nextDouble()-1;s=v1*v1+v2*v2;}while(s>=1||s==0);double m=StrictMath.sqrt(-2*StrictMath.log(s)/s);nextNextGaussian=v2*m;haveNextNextGaussian=true;return v1*m;}
}
