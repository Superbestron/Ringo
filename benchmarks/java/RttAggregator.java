import org.HdrHistogram.Histogram;
import org.apache.logging.log4j.LogManager;
import org.apache.logging.log4j.Logger;

import static org.apache.logging.log4j.util.Unbox.box;

public class RttAggregator implements Runnable {
    private static final Logger log = LogManager.getLogger(RttAggregator.class);
    private static final long INTERVAL = Long.getLong("reportIntervalSec", 5L);
    private static final String TEMPLATE =
            "Latency report generated: interval:[{}s], total:[{}rounds], worst:[{}ns], avg:[{}ns], p99:[{}ns], p999:[{}ns], p9999:[{}ns]";
    public final Histogram histogram;
    private long nowMs;

    public RttAggregator(final Histogram histogram) {
        log.info("Reporting latencies in {}s interval", box(INTERVAL));
        this.histogram = histogram;
    }

    @Override
    public void run() {
        if (System.nanoTime() - nowMs < INTERVAL * 1000) {
            return;
        }
        nowMs = System.nanoTime();
        final var count = histogram.getTotalCount();
        if (count > 0) {
            final long worst = histogram.getMaxValue();
            final long avg = Math.round(histogram.getMean() / 1000);
            final long p99 = histogram.getValueAtPercentile(99) / 1000;
            final long p999 = histogram.getValueAtPercentile(99.9) / 1000;
            final long p9999 = histogram.getValueAtPercentile(99.99) / 1000;
            log.info(TEMPLATE, INTERVAL, box(count), box(worst), box(avg), box(p99), box(p999), box(p9999));
            histogram.reset();
        } else {
            log.warn("No samples collected over the past interval");
        }
    }
}
