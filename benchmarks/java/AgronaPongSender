import org.agrona.MutableDirectBuffer;
import org.agrona.concurrent.ControlledMessageHandler;
import org.agrona.concurrent.ringbuffer.RingBuffer;

import static org.agrona.BitUtil.SIZE_OF_LONG;
import static org.agrona.concurrent.ControlledMessageHandler.Action.COMMIT;

public class AgronaPongSender implements Runnable, ControlledMessageHandler {

    private final RingBuffer sndToRcvBuffer;
    private final RingBuffer rcvToSndBuffer;
    long seq;
    long ts;

    public AgronaPongSender(final RingBuffer sndToRcvBuffer, final RingBuffer rcvToSndBuffer) {
        this.sndToRcvBuffer = sndToRcvBuffer;
        this.rcvToSndBuffer = rcvToSndBuffer;
    }

    @Override
    public void run() {
        while (true) {
            sndToRcvBuffer.controlledRead(this, 500);
        }
    }

    @Override
    public String toString() {
        return "AgronaPongSender";
    }

    @Override
    public Action onMessage(final int msgTypeId, final MutableDirectBuffer buffer, int index, final int length) {
        seq = buffer.getLong(index);
        index += SIZE_OF_LONG;
        ts = buffer.getLong(index);
        while (!AgronaPingSender.write(seq, ts, rcvToSndBuffer));
        return COMMIT;
    }
}
