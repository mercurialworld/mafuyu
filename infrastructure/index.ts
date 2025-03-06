import * as cdk from "aws-cdk-lib";
import { MafuyuStack } from "./stack";

const app = new cdk.App; 
new MafuyuStack(app, "MafuyuStack", {
    env: { account: "575108959833", region: "us-east-1" },
});
