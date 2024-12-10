import org.HdrHistogram.Histogram;
import org.agrona.concurrent.UnsafeBuffer;
import org.agrona.concurrent.ringbuffer.OneToOneRingBuffer;
import org.agrona.concurrent.ringbuffer.RingBuffer;

import java.nio.ByteBuffer;

import static java.nio.ByteBuffer.allocateDirect;
import static org.agrona.concurrent.ringbuffer.RingBufferDescriptor.TRAILER_LENGTH;

public class AgronaPingPongIntegration {

    private static final int MAX_BUF = 1 << 20;
    private static final ByteBuffer recvBuf = allocateRingBufferMemory();
    private static final ByteBuffer sendBuf = allocateRingBufferMemory();
    private static final RingBuffer sndToRcvBuffer = new OneToOneRingBuffer(new UnsafeBuffer(recvBuf));
    private static final RingBuffer rcvToSndBuffer = new OneToOneRingBuffer(new UnsafeBuffer(sendBuf));
    private static final Histogram histogram = new Histogram(3);

    private static ByteBuffer allocateRingBufferMemory() {
        return allocateDirect((MAX_BUF << 1) + TRAILER_LENGTH);
    }

    public static void main(final String[] args) throws InterruptedException {
        if (args.length == 0 || args.length > 2) {
            System.err.println("Usage: AgronaPingPongIntegration <maxInFlights> [reportIntervalSec]");
            return;
        }
        System.setProperty("maxInFlights", args[0]);
        if (args.length == 2) {
            System.setProperty("reportIntervalSec", args[1]);
        }

        final RttAggregator aggregator = new RttAggregator(histogram);
        final AgronaPingSender pingSender = new AgronaPingSender(sndToRcvBuffer, rcvToSndBuffer, aggregator);
        final AgronaPongSender pongSender = new AgronaPongSender(sndToRcvBuffer, rcvToSndBuffer);

        final Thread thread1 = new Thread(pongSender);
        final Thread thread2 = new Thread(pingSender);
        thread1.start();
        thread2.start();

        thread2.join();
        thread1.join();
    }
}
