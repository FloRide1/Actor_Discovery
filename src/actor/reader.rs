use coerce::actor::{Actor, context::ActorContext, LocalActorRef, message::Handler};
use async_trait::async_trait;
use rumqttc::v5::mqttbytes::v5::Packet;
use rumqttc::v5::{AsyncClient, EventLoop};

use crate::message::new_mqtt::NewMQTTMessage;

use super::parser::ParserActor;
use super::persistence::PersistenceActor;


pub struct ReaderActor {
    persistence_addr: LocalActorRef<PersistenceActor>,
    parser_addr: LocalActorRef<ParserActor>,
}

impl ReaderActor {
    pub fn new(persistence_addr: LocalActorRef<PersistenceActor>, parser_addr: LocalActorRef<ParserActor>) -> Self {
        Self {
            persistence_addr,
            parser_addr
        }
    }

}

#[async_trait]
impl Actor for ReaderActor {
    async fn started(&mut self, _ctx: &mut ActorContext) {
        info!("ReaderActor has been created");

        let (client, eventloop) = mqtt_start().await;
        let _ = mqtt_subscribe(&client, "test").await;

        tokio::spawn(event_loop_mqtt(eventloop, _ctx.actor_ref()));
    }
}


async fn mqtt_start() -> (AsyncClient, EventLoop) {
    let mqtt_host = std::env::var("MQTT_HOST").expect("MQTT_HOST is not set");
    info!("MQTT_HOST is set to {}", &mqtt_host);

    let mqtt_port = std::env::var("MQTT_PORT")
        .expect("MQTT_PORT is not set")
        .parse::<u16>()
        .expect("MQTT_PORT is not a number");
    info!("MQTT_PORT is set to {}", mqtt_port);

    let mut mqttoptions = rumqttc::v5::MqttOptions::new("actor_model", &mqtt_host, mqtt_port);
    mqttoptions.set_keep_alive(std::time::Duration::from_secs(5));

    let (client, eventloop) = AsyncClient::new(mqttoptions, 30);
    info!("New MQTT Client \"{}:{}\"", &mqtt_host, mqtt_port);

    (client, eventloop)
}

async fn mqtt_subscribe(client: &AsyncClient, topic: &str) -> Result<(), rumqttc::v5::ClientError> {
    match client
        .subscribe(topic, rumqttc::v5::mqttbytes::QoS::AtMostOnce)
        .await
    {
        Ok(_) => {
            info!("Succesfully subscribe to topic: {}", topic);
            Ok(())
        }

        Err(err) => {
            warn!("Error cannot subscribe to topic: {}", topic);
            Err(err)
        }
    }
}

async fn event_loop_mqtt(mut eventloop: rumqttc::v5::EventLoop, addr: LocalActorRef<ReaderActor>)
{

    loop {
        let notification = eventloop.poll().await;

        if let Err(err) = notification {
            error!("Error = {:?}", err);
            continue;
        }

        let notification = notification.unwrap();
        if let rumqttc::v5::Event::Incoming(Packet::Publish(p)) = notification {
            let topic = std::str::from_utf8(&p.topic).unwrap();
            let content = std::str::from_utf8(&p.payload).unwrap();
            debug!("new message from {}: \"{}\"", topic, content);

            let _ = addr.notify(NewMQTTMessage::new(content.to_string()));

        }
    }
}

#[async_trait::async_trait]
impl Handler<NewMQTTMessage> for ReaderActor {
    async fn handle(&mut self, msg: NewMQTTMessage, _ctx: &mut ActorContext) {
        debug!("ReaderActor receive a new message: '{}'", msg.payload);
        let _ = self.persistence_addr.notify(msg.clone());
        let _ = self.parser_addr.notify(msg.clone());
    }
}
