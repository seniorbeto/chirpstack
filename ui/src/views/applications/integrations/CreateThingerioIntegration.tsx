import { useNavigate } from "react-router-dom";

import { Card } from "antd";

import type { Application } from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";
import {
  ThingerioIntegration,
  CreateThingerioIntegrationRequest,
} from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";

import ThingerioIntegrationForm from "./ThingerioIntegrationForm";
import ApplicationStore from "../../../stores/ApplicationStore";

interface IProps {
  application: Application;
}

function CreateThingerioIntegration(props: IProps) {
  const navigate = useNavigate();

  const onFinish = (obj: ThingerioIntegration) => {
    obj.setApplicationId(props.application.getId());

    const req = new CreateThingerioIntegrationRequest();
    req.setIntegration(obj);

    ApplicationStore.createThingerioIntegration(req, () => {
      navigate(`/tenants/${props.application.getTenantId()}/applications/${props.application.getId()}/integrations`);
    });
  };

  const i = new ThingerioIntegration();

  return (
    <Card title="Add Thingerio integration">
      <ThingerioIntegrationForm initialValues={i} onFinish={onFinish} />
    </Card>
  );
}

export default CreateThingerioIntegration;
