package java.math;
public final class MathContext implements java.io.Serializable {
 public static final MathContext UNLIMITED=new MathContext(0,RoundingMode.HALF_UP), DECIMAL32=new MathContext(7,RoundingMode.HALF_EVEN), DECIMAL64=new MathContext(16,RoundingMode.HALF_EVEN), DECIMAL128=new MathContext(34,RoundingMode.HALF_EVEN);
 private final int precision; private final RoundingMode roundingMode;
 public MathContext(int p){this(p,RoundingMode.HALF_UP);} public MathContext(int p,RoundingMode r){if(p<0)throw new IllegalArgumentException("Digits < 0");if(r==null)throw new NullPointerException();precision=p;roundingMode=r;}
 public int getPrecision(){return precision;}public RoundingMode getRoundingMode(){return roundingMode;}
 public boolean equals(Object o){return o instanceof MathContext&&precision==((MathContext)o).precision&&roundingMode==((MathContext)o).roundingMode;}public int hashCode(){return precision+roundingMode.hashCode()*59;}public String toString(){return "precision="+precision+" roundingMode="+roundingMode;}
}
