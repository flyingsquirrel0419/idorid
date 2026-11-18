package android.view;
import android.content.Context;
import android.util.AttributeSet;
public final class ViewStub extends View {
    public interface OnInflateListener { void onInflate(ViewStub stub,View inflated); }
    private int resource,inflatedId=-1; private LayoutInflater inflater; private OnInflateListener listener; private View inflated;
    public ViewStub(Context context){super(context);super.setVisibility(GONE);} public ViewStub(Context context,int layoutResource){this(context);resource=layoutResource;} public ViewStub(Context context,AttributeSet attrs){super(context,attrs);super.setVisibility(GONE);}
    public void setLayoutResource(int id){resource=id;} public int getLayoutResource(){return resource;} public void setInflatedId(int id){inflatedId=id;} public int getInflatedId(){return inflatedId;} public void setLayoutInflater(LayoutInflater inflater){this.inflater=inflater;} public LayoutInflater getLayoutInflater(){return inflater;} public void setOnInflateListener(OnInflateListener l){listener=l;}
    public View inflate(){if(!(getParent() instanceof ViewGroup))throw new IllegalStateException("ViewStub must have a non-null ViewGroup parent");if(resource==0)throw new IllegalArgumentException("ViewStub must have a valid layoutResource");ViewGroup parent=(ViewGroup)getParent();LayoutInflater f=inflater==null?LayoutInflater.from(getContext()):inflater;View result=f.inflate(resource,parent,false);if(inflatedId!=-1)result.setId(inflatedId);int index=parent.indexOfChild(this);ViewGroup.LayoutParams params=getLayoutParams();parent.removeView(this);if(params!=null)parent.addView(result,index,params);else parent.addView(result,index);inflated=result;if(listener!=null)listener.onInflate(this,result);return result;}
    public void setVisibility(int visibility){if(inflated!=null){inflated.setVisibility(visibility);return;}super.setVisibility(visibility);if(visibility==VISIBLE||visibility==INVISIBLE)inflate();}
}
