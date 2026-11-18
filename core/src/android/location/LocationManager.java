package android.location;
import java.util.ArrayList;
import java.util.List;
public class LocationManager { public static final String GPS_PROVIDER="gps",NETWORK_PROVIDER="network",PASSIVE_PROVIDER="passive"; public boolean isProviderEnabled(String provider){return false;} public boolean isLocationEnabled(){return false;} public Location getLastKnownLocation(String provider){return null;} public List<String> getAllProviders(){return new ArrayList<String>();} }
