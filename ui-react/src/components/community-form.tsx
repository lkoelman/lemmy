import React, { Component } from 'react';
import { Prompt } from 'react-router-dom';
import { Subscription } from 'rxjs';
import { retryWhen, delay, take } from 'rxjs/operators';
import {
  CommunityForm as CommunityFormI,
  UserOperation,
  Category,
  ListCategoriesResponse,
  CommunityResponse,
  GetSiteResponse,
  WebSocketJsonResponse,
} from '../interfaces';
import { WebSocketService } from '../services';
import {
  wsJsonToRes,
  capitalizeFirstLetter,
  toast,
  randomStr,
  setupTribute,
} from '../utils';
import Tribute from 'tributejs/src/Tribute.js';
import autosize from 'autosize';
import { i18n } from '../i18next';

import { Community } from '../interfaces';

interface CommunityFormProps {
  community?: Community; // If a community is given, that means this is an edit
  onCancel?(): any;
  onCreate?(community: Community): any;
  onEdit?(community: Community): any;
}

interface CommunityFormState {
  communityForm: CommunityFormI;
  categories: Array<Category>;
  loading: boolean;
  enable_nsfw: boolean;
}

export class CommunityForm extends Component<
  CommunityFormProps,
  CommunityFormState
> {
  private id = `community-form-${randomStr()}`;
  private tribute: Tribute;
  private subscription: Subscription;

  private emptyState: CommunityFormState = {
    communityForm: {
      name: null,
      title: null,
      category_id: null,
      nsfw: false,
    },
    categories: [],
    loading: false,
    enable_nsfw: null,
  };

  constructor(props: any, context: any) {
    super(props, context);

    this.tribute = setupTribute();
    this.state = this.emptyState;

    if (this.props.community) {
      this.setState({
        communityForm: {
          name: this.props.community.name,
          title: this.props.community.title,
          category_id: this.props.community.category_id,
          description: this.props.community.description,
          edit_id: this.props.community.id,
          nsfw: this.props.community.nsfw,
          auth: null,
        }
      });
    }

    this.subscription = WebSocketService.Instance.subject
      .pipe(retryWhen(errors => errors.pipe(delay(3000), take(10))))
      .subscribe(
        msg => this.parseMessage(msg),
        err => console.error(err),
        () => console.log('complete')
      );

    WebSocketService.Instance.listCategories();
    WebSocketService.Instance.getSite();
  }

  componentDidMount() {
    var textarea: any = document.getElementById(this.id);
    autosize(textarea);
    this.tribute.attach(textarea);
    textarea.addEventListener('tribute-replaced', () => {
      this.state.communityForm.description = textarea.value;
      this.setState(this.state);
      autosize.update(textarea);
    });
  }

  componentWillUnmount() {
    this.subscription.unsubscribe();
  }

  render() {
    return (
      <>
        <Prompt
          when={
            !this.state.loading &&
            Boolean(this.state.communityForm.name ||
              this.state.communityForm.title ||
              this.state.communityForm.description)
          }
          message={i18n.t('block_leaving')}
        />
        <form onSubmit={this.handleCreateCommunitySubmit}>
          <div className="form-group row">
            <label className="col-12 col-form-label" htmlFor="community-name">
              {i18n.t('name')}
            </label>
            <div className="col-12">
              <input
                type="text"
                id="community-name"
                className="form-control"
                value={this.state.communityForm.name}
                onInput={this.handleCommunityNameChange}
                required
                minLength={3}
                maxLength={20}
                pattern="[a-z0-9_]+"
                title={i18n.t('community_reqs')}
              />
            </div>
          </div>

          <div className="form-group row">
            <label className="col-12 col-form-label" htmlFor="community-title">
              {i18n.t('title')}
            </label>
            <div className="col-12">
              <input
                type="text"
                id="community-title"
                value={this.state.communityForm.title}
                onInput={this.handleCommunityTitleChange}
                className="form-control"
                required
                minLength={3}
                maxLength={100}
              />
            </div>
          </div>
          <div className="form-group row">
            <label className="col-12 col-form-label" htmlFor={this.id}>
              {i18n.t('sidebar')}
            </label>
            <div className="col-12">
              <textarea
                id={this.id}
                value={this.state.communityForm.description}
                onInput={this.handleCommunityDescriptionChange}
                className="form-control"
                rows={3}
                maxLength={10000}
              />
            </div>
          </div>
          <div className="form-group row">
            <label className="col-12 col-form-label" htmlFor="community-category">
              {i18n.t('category')}
            </label>
            <div className="col-12">
              <select
                className="form-control"
                id="community-category"
                value={this.state.communityForm.category_id}
                onInput={this.handleCommunityCategoryChange}
              >
                {this.state.categories.map(category => (
                  <option value={category.id}>{category.name}</option>
                ))}
              </select>
            </div>
          </div>

          {this.state.enable_nsfw && (
            <div className="form-group row">
              <div className="col-12">
                <div className="form-check">
                  <input
                    className="form-check-input"
                    id="community-nsfw"
                    type="checkbox"
                    checked={this.state.communityForm.nsfw}
                    onChange={this.handleCommunityNsfwChange}
                  />
                  <label className="form-check-label" htmlFor="community-nsfw">
                    {i18n.t('nsfw')}
                  </label>
                </div>
              </div>
            </div>
          )}
          <div className="form-group row">
            <div className="col-12">
              <button type="submit" className="btn btn-secondary mr-2">
                {this.state.loading ? (
                  <svg className="icon icon-spinner spin">
                    <use xlinkHref="#icon-spinner"></use>
                  </svg>
                ) : this.props.community ? (
                  capitalizeFirstLetter(i18n.t('save'))
                ) : (
                  capitalizeFirstLetter(i18n.t('create'))
                )}
              </button>
              {this.props.community && (
                <button
                  type="button"
                  className="btn btn-secondary"
                  onClick={this.handleCancel}
                >
                  {i18n.t('cancel')}
                </button>
              )}
            </div>
          </div>
        </form>
      </>
    );
  }

  handleCreateCommunitySubmit = (event: any) => {
    event.preventDefault();
    this.setState( { loading: true})
    if (this.props.community) {
      WebSocketService.Instance.editCommunity(this.state.communityForm);
    } else {
      WebSocketService.Instance.createCommunity(this.state.communityForm);
    }
    this.setState(this.state);
  }

  handleCommunityNameChange = (event: any) => {
    this.state.communityForm.name = event.target.value;
    this.setState(this.state);
  }

  handleCommunityTitleChange = (event: any) => {
    this.state.communityForm.title = event.target.value;
    this.setState(this.state);
  }

  handleCommunityDescriptionChange = (event: any) => {
    this.state.communityForm.description = event.target.value;
    this.setState(this.state);
  }

  handleCommunityCategoryChange = (event: any) => {
    this.state.communityForm.category_id = Number(event.target.value);
    this.setState(this.state);
  }

  handleCommunityNsfwChange = (event: any) => {
    this.state.communityForm.nsfw = event.target.checked;
    this.setState(this.state);
  }

  handleCancel = () => {
    this.props.onCancel();
  }

  parseMessage(msg: WebSocketJsonResponse) {
    let res = wsJsonToRes(msg);
    console.log(msg);
    if (msg.error) {
      toast(i18n.t(msg.error), 'danger');
      this.setState({ loading: false});
      return;
    } else if (res.op == UserOperation.ListCategories) {
      let data = res.data as ListCategoriesResponse;
      if (!this.props.community) {
        this.setState({
          communityForm: {
            ...this.state.communityForm,
            category_id: data.categories[0].id
          }
        })
      }
      this.setState({ categories: data.categories});
    } else if (res.op == UserOperation.CreateCommunity) {
      let data = res.data as CommunityResponse;
      this.setState({ loading: false});
      this.props.onCreate(data.community);
    }
    // TODO is this necessary
    else if (res.op == UserOperation.EditCommunity) {
      let data = res.data as CommunityResponse;
      this.setState({ loading: false});
      this.props.onEdit(data.community);
    } else if (res.op == UserOperation.GetSite) {
      let data = res.data as GetSiteResponse;
      this.setState({ enable_nsfw: data.site.enable_nsfw});
    }
  }
}
