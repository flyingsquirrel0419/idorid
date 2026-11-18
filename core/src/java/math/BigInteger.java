package java.math;

/** Immutable signed integer, stored as little-endian base-65536 limbs. */
public class BigInteger extends Number implements Comparable<BigInteger> {
    private final int sign;
    private final int[] mag;
    public static final BigInteger ZERO = new BigInteger(0, new int[0]);
    public static final BigInteger ONE = valueOf(1), TWO = valueOf(2), TEN = valueOf(10);
    private BigInteger(int s, int[] a) {
        int n=a.length; while(n>0 && a[n-1]==0)n--;
        mag=new int[n]; for(int i=0;i<n;i++)mag[i]=a[i]; sign=n==0?0:s;
    }
    public static BigInteger valueOf(long v) {
        if(v==0)return ZERO;
        int s=v<0?-1:1; long x=v<0?-v:v;
        int[] a=new int[4]; for(int i=0;i<4;i++){a[i]=(int)(x&65535);x>>>=16;}
        return new BigInteger(s,a);
    }
    public BigInteger(String s){this(s,10);}
    public BigInteger(String s,int radix){
        if(radix<2||radix>36||s.length()==0)throw new NumberFormatException();
        int p=0,sg=1; char c=s.charAt(0); if(c=='-'||c=='+'){sg=c=='-'?-1:1;p++;}
        if(p==s.length())throw new NumberFormatException();
        BigInteger x=ZERO, r=valueOf(radix);
        for(;p<s.length();p++){int d=Character.digit(s.charAt(p),radix);if(d<0)throw new NumberFormatException();x=x.multiply(r).add(valueOf(d));}
        sign=x.sign*sg;mag=x.mag;
    }
    public int signum(){return sign;}
    public BigInteger abs(){return sign<0?negate():this;}
    public BigInteger negate(){return new BigInteger(-sign,mag);}
    private int cmp(BigInteger b){if(mag.length!=b.mag.length)return mag.length<b.mag.length?-1:1;for(int i=mag.length-1;i>=0;i--)if(mag[i]!=b.mag[i])return mag[i]<b.mag[i]?-1:1;return 0;}
    public int compareTo(BigInteger b){return sign!=b.sign?(sign<b.sign?-1:1):sign*cmp(b);}
    public boolean equals(Object o){return o instanceof BigInteger&&compareTo((BigInteger)o)==0;}
    public int hashCode(){int h=0; for(int i=(mag.length+1)/2-1;i>=0;i--){int j=i*2;int w=mag[j]|(j+1<mag.length?mag[j+1]<<16:0);h=31*h+w;}return h*sign;}
    public BigInteger min(BigInteger b){return compareTo(b)<=0?this:b;} public BigInteger max(BigInteger b){return compareTo(b)>=0?this:b;}
    public BigInteger add(BigInteger b){
        if(sign==0)return b;if(b.sign==0)return this;
        if(sign==b.sign){int n=Math.max(mag.length,b.mag.length);int[] a=new int[n+1];int carry=0;for(int i=0;i<n;i++){int v=carry+(i<mag.length?mag[i]:0)+(i<b.mag.length?b.mag[i]:0);a[i]=v&65535;carry=v>>>16;}a[n]=carry;return new BigInteger(sign,a);}
        int c=cmp(b);if(c==0)return ZERO;BigInteger x=c>0?this:b,y=c>0?b:this;int[] a=new int[x.mag.length];int borrow=0;for(int i=0;i<a.length;i++){int v=x.mag[i]-(i<y.mag.length?y.mag[i]:0)-borrow;borrow=v<0?1:0;a[i]=v&65535;}return new BigInteger(x.sign,a);
    }
    public BigInteger subtract(BigInteger b){return add(b.negate());}
    public BigInteger multiply(BigInteger b){if(sign==0||b.sign==0)return ZERO;int[] a=new int[mag.length+b.mag.length];for(int i=0;i<mag.length;i++){long carry=0;for(int j=0;j<b.mag.length;j++){long v=(long)mag[i]*b.mag[j]+a[i+j]+carry;a[i+j]=(int)v&65535;carry=v>>>16;}a[i+b.mag.length]=(int)carry;}return new BigInteger(sign*b.sign,a);}
    public BigInteger shiftLeft(int n){if(n<0)return shiftRight(-n);if(sign==0)return this;int w=n/16,b=n%16;int[] a=new int[mag.length+w+1];int c=0;for(int i=0;i<mag.length;i++){int v=(mag[i]<<b)|c;a[i+w]=v&65535;c=v>>>16;}a[mag.length+w]=c;return new BigInteger(sign,a);}
    public BigInteger shiftRight(int n){if(n<0)return shiftLeft(-n);if(n==0||sign==0)return this;int w=n/16,b=n%16;if(w>=mag.length)return sign<0?valueOf(-1):ZERO;int[] a=new int[mag.length-w];int c=0;for(int i=mag.length-1;i>=w;i--){int v=mag[i];a[i-w]=(v>>>b)|c;c=b==0?0:(v&((1<<b)-1))<<(16-b);}BigInteger x=new BigInteger(1,a);if(sign<0){boolean lost=c!=0;for(int i=0;i<w;i++)lost|=mag[i]!=0;if(lost)x=x.add(ONE);return x.negate();}return x;}
    public BigInteger[] divideAndRemainder(BigInteger b){
        if(b.sign==0)throw new ArithmeticException("BigInteger divide by zero");
        BigInteger d=b.abs(),r=abs();if(r.cmp(d)<0)return new BigInteger[]{ZERO,this};
        int shift=r.bitLength()-d.bitLength();BigInteger t=d.shiftLeft(shift);int[] q=new int[shift/16+1];
        for(int i=shift;i>=0;i--){if(r.cmp(t)>=0){r=r.subtract(t);q[i/16]|=1<<(i%16);}t=t.shiftRight(1);}
        return new BigInteger[]{new BigInteger(sign*b.sign,q),sign<0?r.negate():r};
    }
    public BigInteger divide(BigInteger b){return divideAndRemainder(b)[0];}public BigInteger remainder(BigInteger b){return divideAndRemainder(b)[1];}
    public BigInteger mod(BigInteger b){if(b.sign<=0)throw new ArithmeticException("BigInteger: modulus not positive");BigInteger r=remainder(b);return r.sign<0?r.add(b):r;}
    public BigInteger pow(int n){if(n<0)throw new ArithmeticException("Negative exponent");BigInteger a=this,r=ONE;while(n!=0){if((n&1)!=0)r=r.multiply(a);n>>>=1;if(n!=0)a=a.multiply(a);}return r;}
    public BigInteger gcd(BigInteger b){BigInteger a=abs();b=b.abs();while(b.sign!=0){BigInteger r=a.remainder(b);a=b;b=r;}return a;}
    public int bitLength(){if(sign==0)return 0;if(sign<0)return negate().subtract(ONE).bitLength();return (mag.length-1)*16+32-Integer.numberOfLeadingZeros(mag[mag.length-1]);}
    public int bitCount(){BigInteger x=sign<0?negate().subtract(ONE):this;int n=0;for(int i=0;i<x.mag.length;i++)n+=Integer.bitCount(x.mag[i]);return n;}
    public boolean testBit(int n){if(n<0)throw new ArithmeticException("Negative bit address");return shiftRight(n).intValue()%2!=0;}
    private BigInteger bitop(BigInteger b,int op){int n=Math.max(mag.length,b.mag.length)+1;int[] a=new int[n];BigInteger x=this,y=b;for(int i=0;i<n;i++){int u=x.intValue()&65535,v=y.intValue()&65535;a[i]=op==0?u&v:op==1?u|v:u^v;x=x.shiftRight(16);y=y.shiftRight(16);}if((a[n-1]&32768)==0)return new BigInteger(1,a);for(int i=0;i<n;i++)a[i]^=65535;return new BigInteger(1,a).add(ONE).negate();}
    public BigInteger and(BigInteger b){return bitop(b,0);}public BigInteger or(BigInteger b){return bitop(b,1);}public BigInteger xor(BigInteger b){return bitop(b,2);}public BigInteger not(){return negate().subtract(ONE);}
    public int intValue(){return (int)longValue();}public long longValue(){long x=0;for(int i=Math.min(4,mag.length)-1;i>=0;i--)x=(x<<16)|mag[i];return sign<0?-x:x;}public long longValueExact(){long x=longValue();if(!valueOf(x).equals(this))throw new ArithmeticException("BigInteger out of long range");return x;}public float floatValue(){return (float)doubleValue();}public double doubleValue(){double x=0;for(int i=mag.length-1;i>=0;i--)x=x*65536.0+mag[i];return sign<0?-x:x;}
    public String toString(){return toString(10);}public String toString(int radix){if(radix<2||radix>36)radix=10;if(sign==0)return "0";String digits="0123456789abcdefghijklmnopqrstuvwxyz";int[] a=new int[mag.length];for(int i=0;i<a.length;i++)a[i]=mag[i];StringBuilder s=new StringBuilder();int n=a.length;while(n>0){int r=0;for(int i=n-1;i>=0;i--){int v=(r<<16)+a[i];a[i]=v/radix;r=v%radix;}s.append(digits.charAt(r));while(n>0&&a[n-1]==0)n--;}if(sign<0)s.append('-');return s.reverse().toString();}
    public BigInteger sqrt(){if(sign<0)throw new ArithmeticException("Negative BigInteger");if(sign==0)return ZERO;BigInteger x=ONE.shiftLeft((bitLength()+1)/2);while(true){BigInteger y=x.add(divide(x)).shiftRight(1);if(y.compareTo(x)>=0)return x;x=y;}}
    public BigInteger modPow(BigInteger e,BigInteger m){if(m.sign<=0)throw new ArithmeticException("BigInteger: modulus not positive");if(e.sign<0)return modInverse(m).modPow(e.negate(),m);BigInteger r=ONE.mod(m),a=mod(m);while(e.sign!=0){if(e.testBit(0))r=r.multiply(a).mod(m);e=e.shiftRight(1);if(e.sign!=0)a=a.multiply(a).mod(m);}return r;}
    public BigInteger modInverse(BigInteger m){if(m.sign<=0)throw new ArithmeticException("BigInteger: modulus not positive");BigInteger a=mod(m),b=m,x=ONE,y=ZERO;while(b.sign!=0){BigInteger[] qr=a.divideAndRemainder(b);BigInteger t=x.subtract(qr[0].multiply(y));a=b;b=qr[1];x=y;y=t;}if(!a.equals(ONE))throw new ArithmeticException("BigInteger not invertible.");return x.mod(m);}
    public boolean isProbablePrime(int certainty){if(certainty<=0)return true;BigInteger n=abs();if(n.compareTo(TWO)<0)return false;int[] bases={2,3,5,7,11,13,17,19,23,29,31,37};for(int p:bases){BigInteger b=valueOf(p);if(n.equals(b))return true;if(n.mod(b).sign==0)return false;}BigInteger d=n.subtract(ONE);int s=0;while(!d.testBit(0)){d=d.shiftRight(1);s++;}int rounds=Math.min(bases.length,Math.max(1,(certainty+1)/2));for(int i=0;i<rounds;i++){BigInteger x=valueOf(bases[i]).modPow(d,n);if(x.equals(ONE)||x.equals(n.subtract(ONE)))continue;boolean ok=false;for(int j=1;j<s;j++){x=x.multiply(x).mod(n);if(x.equals(n.subtract(ONE))){ok=true;break;}}if(!ok)return false;}return true;}
}
