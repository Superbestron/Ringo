import org.agrona.MutableDirectBuffer;
import org.agrona.collections.LongHashSet;
import org.agrona.concurrent.AtomicBuffer;
import org.agrona.concurrent.ControlledMessageHandler;
import org.agrona.concurrent.ringbuffer.RingBuffer;
import org.apache.logging.log4j.LogManager;
import org.apache.logging.log4j.Logger;

import static org.agrona.BitUtil.SIZE_OF_LONG;
import static org.agrona.concurrent.ControlledMessageHandler.Action.COMMIT;
import static org.apache.logging.log4j.util.Unbox.box;

public class AgronaPingSender implements Runnable, ControlledMessageHandler {

    static final int MAX_IN_FLIGHTS = Integer.getInteger("maxInFlights", 1000);
    private static final Logger log = LogManager.getLogger(AgronaPingSender.class);
    private static final Logger throttledLog = new ThrottledLogger(log, 3000);
    private long seq = 1;
    private final LongHashSet pending = new LongHashSet();
    private final RttAggregator aggregator;
    private final RingBuffer sndToRcvBuffer;
    private final RingBuffer rcvToSndBuffer;

    public AgronaPingSender(final RingBuffer sndToRcvBuffer, final RingBuffer rcvToSndBuffer, final RttAggregator aggregator) {
        this.sndToRcvBuffer = sndToRcvBuffer;
        this.rcvToSndBuffer = rcvToSndBuffer;
        this.aggregator = aggregator;
        log.info("Max In Flights: {}", box(MAX_IN_FLIGHTS));
    }

    @Override
    public void run() {
        while (true) {
            if (pending.size() < MAX_IN_FLIGHTS) {
                if (write(seq, System.nanoTime(), sndToRcvBuffer)) {
                    pending.add(seq);
                    seq++;
                }
            }
            rcvToSndBuffer.controlledRead(this, 500);

            aggregator.run();
        }
    }

    public static boolean write(final long seq, final long nowNs, final RingBuffer ringBuffer) {
        final int index = ringBuffer.tryClaim(1, 2 * SIZE_OF_LONG);
        if (index > 0) {
            try {
                final AtomicBuffer buffer = ringBuffer.buffer();  // Work with the buffer directly using the index
                int offset = index;
                buffer.putLong(offset, seq);
                offset += SIZE_OF_LONG;
                buffer.putLong(offset, nowNs);
                ringBuffer.commit(index); // commit message
                return true;
            } catch (final Exception ex) {
                log.error("Exception while writing message to {}: {}", ex, ringBuffer);
                ringBuffer.abort(index); // allow consumer to proceed
                return false;
            }
        }
        return false;
    }

    @Override
    public String toString() {
        return "AgronaPingSender";
    }

    @Override
    public Action onMessage(final int msgTypeId, final MutableDirectBuffer buffer, int index, final int length) {
        final long seq = buffer.getLong(index);
        index += SIZE_OF_LONG;
        final long ts = buffer.getLong(index);
        aggregator.histogram.recordValue(Clock.system().epochNs() - ts);
        pending.remove(seq);
        return COMMIT; // return after reading a message
    }
}
