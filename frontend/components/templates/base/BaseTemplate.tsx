export interface IBaseTemplate {
  text: string;
}

const BaseTemplate: React.FC<IBaseTemplate> = ({ text }) => {
  return <div>{text}</div>;
};

export default BaseTemplate;
