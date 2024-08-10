import React from "react";
import ParameterValue from "./parameterValue";

class ParameterTree extends React.Component {
  constructor(props) {
    super(props);
    this.level = props.level || 0;

    console.log("init ParameterTree", this.props);
  }

  render() {
    const { type, value } = this.props;
    const level = this.level;
    const isArray = type == "Array" && value.length;
    if (!isArray) {
      return <ParameterValue type={type} value={value}></ParameterValue>;
    }
    return (
      <div>
        {value.map((item, i) => (
          <ParameterTree
            key={i}
            type={item.type}
            value={item.value}
            level={level + 1}
          ></ParameterTree>
        ))}
      </div>
    );
  }
}

export default ParameterTree;
