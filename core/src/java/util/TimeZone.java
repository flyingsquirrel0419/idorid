package java.util;
public class TimeZone implements java.io.Serializable,Cloneable {
 private final String id;private final int offset;private static TimeZone current=new TimeZone("UTC",0);
 protected TimeZone(String id,int offset){this.id=id;this.offset=offset;}
 public static TimeZone getTimeZone(String id){if(id==null)throw new NullPointerException();if(id.equals("UTC")||id.equals("GMT")||id.equals("Etc/UTC"))return new TimeZone(id,0);if(id.startsWith("GMT+")||id.startsWith("GMT-")){try{String x=id.substring(4);int c=x.indexOf(':');int h=c<0?Integer.parseInt(x):Integer.parseInt(x.substring(0,c));int m=c<0?0:Integer.parseInt(x.substring(c+1));if(h<=23&&m<=59)return new TimeZone(id,(id.charAt(3)=='-'?-1:1)*(h*60+m)*60000);}catch(RuntimeException e){}}return new TimeZone("GMT",0);}
 public static TimeZone getDefault(){return current;}public static void setDefault(TimeZone z){current=z==null?getTimeZone("UTC"):z;}public String getID(){return id;}public int getRawOffset(){return offset;}public int getOffset(long time){return offset;}public boolean useDaylightTime(){return false;}public boolean inDaylightTime(Date d){return false;}
}
