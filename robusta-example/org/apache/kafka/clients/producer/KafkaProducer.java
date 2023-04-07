package org.apache.kafka.clients.producer;

public class KafkaProducer {
    public native void init(String bootstrapServers, boolean useSsl);

    public native void send(String bootstrapServers, String topic, String key, String payload);

    public native void close(String bootstrapServers);
}
