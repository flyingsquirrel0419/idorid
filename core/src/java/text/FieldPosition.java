package java.text;
public class FieldPosition {private final int field;private int begin,end;public FieldPosition(int f){field=f;}public int getField(){return field;}public int getBeginIndex(){return begin;}public int getEndIndex(){return end;}public void setBeginIndex(int i){begin=i;}public void setEndIndex(int i){end=i;}}
