import React, { Component } from 'react';
import { CommunityForm } from './community-form';
import { Community } from '../../interfaces';
import { WebSocketService } from '../../services';
import { i18n } from '../../i18next';

export class CreateCommunity extends Component<any, any> {
  constructor(props: any, context: any) {
    super(props, context);
    this.handleCommunityCreate = this.handleCommunityCreate.bind(this);
  }

  componentDidMount() {
    document.title = `${i18n.t('create_community')} - ${
      WebSocketService.Instance.site.name
    }`;
  }

  render() {
    return (
      <div className="container">
        <div className="row">
          <div className="col-12 col-lg-6 offset-lg-3 mb-4">
            <h5>{i18n.t('create_community')}</h5>
            <CommunityForm onCreate={this.handleCommunityCreate} />
          </div>
        </div>
      </div>
    );
  }

  handleCommunityCreate(community: Community) {
    this.props.history.push(`/c/${community.name}`);
  }
}
