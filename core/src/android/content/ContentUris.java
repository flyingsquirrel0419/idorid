package android.content;
public class ContentUris { public static long parseId(android.net.Uri uri) { String s=uri.toString(); return Long.parseLong(s.substring(s.lastIndexOf('/')+1)); } public static android.net.Uri withAppendedId(android.net.Uri uri,long id) { return android.net.Uri.parse(uri.toString()+"/"+id); } }
