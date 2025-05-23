use derive_builder::Builder;
use prost::Message;
use prost::bytes::Bytes;
use std::borrow::Cow;
use xmtp_proto::traits::{BodyError, Endpoint};
use xmtp_proto::xmtp::xmtpv4::envelopes::ClientEnvelope;
use xmtp_proto::xmtp::xmtpv4::payer_api::FILE_DESCRIPTOR_SET;
use xmtp_proto::xmtp::xmtpv4::payer_api::{
    PublishClientEnvelopesRequest, PublishClientEnvelopesResponse,
};

#[derive(Debug, Builder, Default)]
#[builder(setter(strip_option), build_fn(error = "BodyError"))]
pub struct PublishClientEnvelopes {
    #[builder(setter(each(name = "envelope", into)))]
    envelopes: Vec<ClientEnvelope>,
}

impl PublishClientEnvelopes {
    pub fn builder() -> PublishClientEnvelopesBuilder {
        Default::default()
    }
}

impl Endpoint for PublishClientEnvelopes {
    type Output = PublishClientEnvelopesResponse;
    fn http_endpoint(&self) -> Cow<'static, str> {
        Cow::from("/mls/v2/payer/publish-client-envelopes")
    }

    fn grpc_endpoint(&self) -> Cow<'static, str> {
        crate::path_and_query::<PublishClientEnvelopesRequest>(FILE_DESCRIPTOR_SET)
    }

    fn body(&self) -> Result<Bytes, BodyError> {
        Ok(PublishClientEnvelopesRequest {
            envelopes: self.envelopes.clone(),
        }
        .encode_to_vec()
        .into())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use xmtp_proto::prelude::*;

    #[xmtp_common::test]
    fn test_file_descriptor() {
        use xmtp_proto::xmtp::xmtpv4::payer_api::{
            FILE_DESCRIPTOR_SET, PublishClientEnvelopesRequest,
        };

        let pnq = crate::path_and_query::<PublishClientEnvelopesRequest>(FILE_DESCRIPTOR_SET);
        println!("{}", pnq);
    }

    #[xmtp_common::test]
    async fn test_publish_client_envelopes() {
        use xmtp_proto::xmtp::xmtpv4::envelopes::ClientEnvelope;

        let client = crate::TestClient::create_local_d14n();
        let client = client.build().await.unwrap();

        let endpoint = PublishClientEnvelopes::builder()
            .envelopes(vec![ClientEnvelope::default()])
            .build()
            .unwrap();

        let result = endpoint.query(&client).await;
        assert!(result.is_err());
    }
}
