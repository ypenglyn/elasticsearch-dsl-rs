use std::convert::TryFrom;

/// Floating point number between `0` and `1.0` used to increase the
/// [relevance scores](https://www.elastic.co/guide/en/elasticsearch/reference/current/query-filter-context.html#relevance-scores)
/// of documents matching multiple query clauses. Defaults to `0.0`.
///
/// You can use the `tie_breaker` value to assign higher relevance scores to
/// documents that contain the same term in multiple fields than documents that
/// contain this term in only the best of those multiple fields, without
/// confusing this with the better case of two different terms in the multiple
/// fields.
///
/// If a document matches multiple clauses, the `dis_max` query calculates
/// the relevance score for the document as follows:
/// 1. Take the relevance score from a matching clause with the highest score.
/// 2. Multiply the score from any other matching clauses by the tie_breaker value.
/// 3. Add the highest score to the multiplied scores.
///
/// If the `tie_breaker` value is greater than `0.0`, all matching clauses
/// count, but the clause with the highest score counts most.
#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub struct TieBreaker(f32);

impl TieBreaker {
    /// Minimum value
    const MINIMUM: f32 = 0f32;

    /// Maximum value
    const MAXIMUM: f32 = 1f32;
}

impl TryFrom<f32> for TieBreaker {
    type Error = &'static str;

    fn try_from(value: f32) -> Result<Self, Self::Error> {
        if value < Self::MINIMUM {
            Err("Value cannot be lower than 0")
        } else if value > Self::MAXIMUM {
            Err("Value cannot be greater than 1")
        } else {
            Ok(Self(value))
        }
    }
}
