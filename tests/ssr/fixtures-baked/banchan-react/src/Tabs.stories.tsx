import type { Meta, StoryObj } from "@storybook/react";
import { Tabs, TabList, Tab, TabPanel } from "./Tabs";

const meta: Meta<typeof Tabs> = { component: Tabs };
export default meta;
type Story = StoryObj<typeof Tabs>;

export const Default: Story = {
  render: () => (
    <Tabs defaultValue="account">
      <TabList>
        <Tab value="account">계정</Tab>
        <Tab value="notifications">알림</Tab>
        <Tab value="security">보안</Tab>
      </TabList>
      <TabPanel value="account">계정 정보 내용</TabPanel>
      <TabPanel value="notifications">알림 설정 내용</TabPanel>
      <TabPanel value="security">보안 설정 내용</TabPanel>
    </Tabs>
  ),
};
