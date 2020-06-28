import React, { Component } from 'react';
import { DataType } from '../../interfaces';

import { i18n } from '../../i18next';

interface DataTypeSelectProps {
  type_: DataType;
  onChange?(val: DataType): any;
}

interface DataTypeSelectState {
  type_: DataType;
}

export class DataTypeSelect extends Component<
  DataTypeSelectProps,
  DataTypeSelectState
> {

  state : DataTypeSelectState;
  
  private emptyState: DataTypeSelectState = {
    type_: this.props.type_,
  };

  constructor(props: any, context: any) {
    super(props, context);
    this.state = this.emptyState;
  }

  render() {
    return (
      <div className="btn-group btn-group-toggle">
        <label
          className={`pointer btn btn-sm btn-secondary 
            ${this.state.type_ == DataType.Post && 'active'}
          `}
        >
          <input
            type="radio"
            value={DataType.Post}
            checked={this.state.type_ == DataType.Post}
            onChange={this.handleTypeChange}
          />
          {i18n.t('posts')}
        </label>
        <label
          className={`pointer btn btn-sm btn-secondary ${this.state.type_ ==
            DataType.Comment && 'active'}`}
        >
          <input
            type="radio"
            value={DataType.Comment}
            checked={this.state.type_ == DataType.Comment}
            onChange={this.handleTypeChange}
          />
          {i18n.t('comments')}
        </label>
      </div>
    );
  }

  handleTypeChange= (event: any) => {
    this.state.type_ = Number(event.target.value);
    this.setState(this.state);
    this.props.onChange(this.state.type_);
  }
}
