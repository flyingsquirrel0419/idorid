package android.text;
public interface TextDirectionHeuristic { boolean isRtl(CharSequence text,int start,int count); boolean isRtl(char[] text,int start,int count); }
