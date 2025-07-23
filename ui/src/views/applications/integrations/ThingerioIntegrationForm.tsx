import { Form, Input, Button } from "antd";

import { ThingerioIntegration } from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";

import { onFinishFailed } from "../../helpers";

interface IProps {
  initialValues: ThingerioIntegration;
  onFinish: (obj: ThingerioIntegration) => void;
}

function ThingerioIntegrationForm(props: IProps) {
  const onFinish = (values: ThingerioIntegration.AsObject) => {
    const v = Object.assign(props.initialValues.toObject(), values);
    const i = new ThingerioIntegration();

    i.setApplicationId(v.applicationId);
    i.setServer(v.server);
    i.setToken(v.token);

    props.onFinish(i);
  };

  return (
    <Form
      layout="vertical"
      initialValues={props.initialValues.toObject()}
      onFinish={onFinish}
      onFinishFailed={onFinishFailed}
    >
      <Form.Item
        label="Thingerio server"
        name="server"
        rules={[{ required: true, message: "Please enter a Thingerio server!" }]}
      >
        <Input placeholder="https://host:port" />
      </Form.Item>
      <Form.Item
        label="Authentication token"
        name="token"
        rules={[{ required: true, message: "Please enter a Thingerio token!" }]}
      >
        <Input.Password />
      </Form.Item>
      <Form.Item>
        <Button type="primary" htmlType="submit">
          Submit
        </Button>
      </Form.Item>
    </Form>
  );
}

export default ThingerioIntegrationForm;
