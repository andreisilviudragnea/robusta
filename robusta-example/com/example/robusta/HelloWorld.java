package com.example.robusta;

import java.util.*;
import org.apache.kafka.clients.producer.KafkaProducer;

class HelloWorld {
    private String foo = "";

    private static native ArrayList<String> special(ArrayList<Integer> input1, int in2);

    // pub extern "java" fn staticJavaAdd(i: i32, u: i32) -> i32 {}
    public static int staticJavaAdd(int i, int u) {
        return i + u;
    }

    // pub extern "jni" fn catchMe(self, _env: &JNIEnv) -> JniResult<i32>
    private native void catchMe() throws IllegalArgumentException;

    // pub extern "java" fn javaAdd(&self, i: i32, u: i32) -> i32 {}
    public int javaAdd(int i, int u) {
        return i + u;
    }

    public String javaAdd(String i, int f, String u) {
            return i + u;
    }

    // pub extern "jni" fn nativeFun(self, static_call: bool) -> i32
    public native int nativeFun(boolean staticCall);

    static {
        System.loadLibrary("robusta_example");
    }

    private native void setStringHelloWorld();

    public static void main(String[] args) throws InterruptedException {
        ArrayList<String> output = HelloWorld.special(new ArrayList<Integer>(List.of(1, 2, 3)), 4);
        System.out.println(output);

        HelloWorld h = new HelloWorld();
        System.out.println(h.nativeFun(false));
        System.out.println(h.nativeFun(true));

        try {
            h.catchMe();
        } catch (IllegalArgumentException e) {
            System.out.println("Caught exception. Message: \"" + e.getMessage() + "\"");
            System.out.println("Printing stacktrace:");
            e.printStackTrace();
        }

        System.out.println("Now h.foo is: \"" + h.foo + "\"");
        h.setStringHelloWorld();
        System.out.println("After setStringHelloWorld() h.foo is: \"" + h.foo + "\"");

        var kafkaProducer = new KafkaProducer();

        kafkaProducer.init("localhost:9092", false);
        kafkaProducer.send("localhost:9092", "quickstart-events", "key", "payload");
        Thread.sleep(10_000);
        kafkaProducer.close("localhost:9092");
	}
}
