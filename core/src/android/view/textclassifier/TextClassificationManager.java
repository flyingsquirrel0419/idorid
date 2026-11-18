package android.view.textclassifier;
public class TextClassificationManager { private TextClassifier classifier=TextClassifier.NO_OP; public TextClassifier getTextClassifier(){return classifier;} public void setTextClassifier(TextClassifier c){classifier=c==null?TextClassifier.NO_OP:c;} }
