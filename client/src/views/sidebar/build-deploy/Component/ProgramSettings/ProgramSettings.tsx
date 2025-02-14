import { FC, ReactNode } from "react";
import styled, { css } from "styled-components";

import Foldable from "../../../../../components/Foldable";
import IDL from "./IDL";
import ProgramBinary from "./ProgramBinary";
import ProgramID from "./ProgramID";

/** All program settings */
const PROGRAM_SETTINGS: ProgramSettingProps[] = [
  {
    title: "Program ID",
    description:
      "Import/export program keypair or input a public key for the program.",
    element: <ProgramID />,
    open: true,
  },
  {
    title: "Program binary",
    description: "Import your program and deploy without failure.",
    element: <ProgramBinary />,
  },
  {
    title: "IDL",
    description: "Anchor IDL interactions.",
    element: <IDL />,
  },
];

const ProgramSettings = () => (
  <Wrapper>
    {PROGRAM_SETTINGS.map((setting) => (
      <ProgramSetting key={setting.title} {...setting} />
    ))}
  </Wrapper>
);

const Wrapper = styled.div`
  display: flex;
  flex-direction: column;
  justify-content: center;
  margin-top: 1rem;
  border-top: 1px solid ${({ theme }) => theme.colors.default.border};
`;

interface ProgramSettingProps {
  /** Title text that will be shown as a foldable title */
  title: string;
  /** Description of the setting that will be shown after unfolding */
  description: string;
  /** Component that will be shown inside the foldable and under the description */
  element: ReactNode;
  /** Whether the foldable is open by default */
  open?: boolean;
}

const ProgramSetting: FC<ProgramSettingProps> = ({
  title,
  description,
  element,
  open,
}) => (
  <ProgramSettingWrapper>
    <Foldable
      open={open}
      element={<ProgramSettingTitle>{title}</ProgramSettingTitle>}
    >
      <ProgramSettingDescription>{description}</ProgramSettingDescription>
      <ProgramSettingContent>{element}</ProgramSettingContent>
    </Foldable>
  </ProgramSettingWrapper>
);

const ProgramSettingWrapper = styled.div`
  margin-top: 1rem;
  margin-left: 0.5rem;
`;

const ProgramSettingTitle = styled.span``;

const ProgramSettingDescription = styled.div`
  ${({ theme }) => css`
    color: ${theme.colors.default.textSecondary};
    font-size: ${theme.font.code.size.small};
  `}
`;

const ProgramSettingContent = styled.div`
  margin-top: 0.5rem;
`;

export default ProgramSettings;
