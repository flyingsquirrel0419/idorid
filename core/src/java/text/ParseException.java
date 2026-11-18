package java.text;
public class ParseException extends Exception {private final int errorOffset;public ParseException(String s,int offset){super(s);errorOffset=offset;}public int getErrorOffset(){return errorOffset;}}
