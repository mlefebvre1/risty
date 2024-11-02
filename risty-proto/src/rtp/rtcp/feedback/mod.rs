mod nack;
mod rist;

/// This field identifies the type of the feedback message
enum Subtype {
    RangeBasedNACK = 0,
    GenericNack = 1,
    EchoRequest = 2,
    EchoResponse = 3,
}
