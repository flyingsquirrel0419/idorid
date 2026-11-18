package java.math;
public enum RoundingMode { UP, DOWN, CEILING, FLOOR, HALF_UP, HALF_DOWN, HALF_EVEN, UNNECESSARY;
 public static RoundingMode valueOf(int i){if(i<0||i>=values().length)throw new IllegalArgumentException("argument out of range");return values()[i];}
}
