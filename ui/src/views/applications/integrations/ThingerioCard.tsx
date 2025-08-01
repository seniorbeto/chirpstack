import { Link } from "react-router-dom";

import { Col, Card, Popconfirm } from "antd";
import { PlusOutlined, EditOutlined, DeleteOutlined } from "@ant-design/icons";

import type { Application } from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";
import { DeleteThingerioIntegrationRequest } from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";

import ApplicationStore from "../../../stores/ApplicationStore";

interface IProps {
  application: Application;
  add?: boolean;
}

function ThingerCard(props: IProps) {
  const onDelete = () => {
    const req = new DeleteThingerioIntegrationRequest();
    req.setApplicationId(props.application.getId());
    ApplicationStore.deleteThingerioIntegration(req, () => {});
  };

  let actions: JSX.Element[] = [];

  if (props.add) {
    actions = [
      <Link to="thingerio/create">
        <PlusOutlined />
      </Link>,
    ];
  } else {
    actions = [
      <Link to="thingerio/edit">
        <EditOutlined />
      </Link>,
      <Popconfirm title="Are you sure you want to delete this integration?" onConfirm={onDelete}>
        <DeleteOutlined />
      </Popconfirm>,
    ];
  }

  return (
    <Col span={8}>
      <Card
        title="Thinger.io"
        className="integration-card"
        cover={<img alt="thinger" src="/integrations/thinger_logo_white.png" style={{ padding: 1 }} />}
        actions={actions}
      >
        <Card.Meta description="The Thinger integration enables seamless forwarding messages to a Thinger.io instance" />
      </Card>
    </Col>
  );
}

export default ThingerCard;
