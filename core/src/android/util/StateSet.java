package android.util;
public class StateSet {
    public static final int[] WILD_CARD=new int[0], NOTHING={0};
    public static boolean isWildCard(int[] stateSpec) { return stateSpec.length==0 || stateSpec[0]==0; }
    public static boolean stateSetMatches(int[] spec,int[] states) { for(int want:spec) { if(want==0) break; boolean must=want>0,found=false; int id=must?want:-want; if(states!=null)for(int state:states) { if(state==0)break; if(state==id){found=true;break;} } if(must!=found)return false; } return true; }
    public static boolean stateSetMatches(int[] spec,int state) { return stateSetMatches(spec,new int[]{state}); }
    public static int[] trimStateSet(int[] states,int size) { if(states.length==size)return states; int[] out=new int[size]; System.arraycopy(states,0,out,0,size); return out; }
}
