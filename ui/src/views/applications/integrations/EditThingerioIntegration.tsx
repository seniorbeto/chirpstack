import { useState, useEffect } from "react";
import { useNavigate } from "react-router-dom";

import { Card } from "antd";

import type {
  Application,
  ThingerioIntegration,
  GetThingerioIntegrationResponse,
} from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";
import {
  GetThingerioIntegrationRequest,
  UpdateThingerioIntegrationRequest,
} from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";

import ThingerioIntegrationForm from "./ThingerioIntegrationForm";
import ApplicationStore from "../../../stores/ApplicationStore";

interface IProps {
  application: Application;
}

function EditThingerioIntegration(props: IProps) {
  const navigate = useNavigate();
  const [integration, setIntegration] = useState<ThingerioIntegration | undefined>(undefined);

  useEffect(() => {
    const req = new GetThingerioIntegrationRequest();
    req.setApplicationId(props.application.getId());

    ApplicationStore.getThingerioIntegration(req, (resp: GetThingerioIntegrationResponse) => {
      setIntegration(resp.getIntegration());
    });
  }, [props]);

  const onFinish = (obj: ThingerioIntegration) => {
    const req = new UpdateThingerioIntegrationRequest();
    req.setIntegration(obj);

    ApplicationStore.updateThingerioIntegration(req, () => {
      navigate(`/tenants/${props.application.getTenantId()}/applications/${props.application.getId()}/integrations`);
    });
  };

  if (integration === undefined) {
    return null;
  }

  return (
    <Card title="Update Pilot Things integration">
      <ThingerioIntegrationForm initialValues={integration} onFinish={onFinish} />
    </Card>
  );
}

export default EditThingerioIntegration;
