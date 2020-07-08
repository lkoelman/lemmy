import React, { Component } from 'react';
import { Prompt } from 'react-router-dom';
import { Site, SiteForm as SiteFormI } from '../../interfaces';
import { WebSocketService } from '../../services';
import { capitalizeFirstLetter, randomStr, setupTribute } from '../../utils';
import autosize from 'autosize';
import Tribute from 'tributejs/src/Tribute'; // prevent type specification
import { i18n } from '../../i18next';

interface SiteFormProps {
  site?: Site; // If a site is given, that means this is an edit
  onCancel?(): any;
}

interface SiteFormState {
  siteForm: SiteFormI;
  loading: boolean;
}

/**
 * Form for creating a community.
 */
export class SiteForm extends Component<SiteFormProps, SiteFormState> {

  state: SiteFormState;

  private id = `site-form-${randomStr()}`;
  private tribute: Tribute;
  private emptyState: SiteFormState = {
    siteForm: {
      enable_downvotes: true,
      open_registration: true,
      enable_nsfw: true,
      name: null,
    },
    loading: false,
  };

  constructor(props: any, context: any) {
    super(props, context);

    this.tribute = setupTribute();
    this.state = this.emptyState;

    if (this.props.site) {
      this.state.siteForm = {
        name: this.props.site.name,
        description: this.props.site.description,
        enable_downvotes: this.props.site.enable_downvotes,
        open_registration: this.props.site.open_registration,
        enable_nsfw: this.props.site.enable_nsfw,
      };
    }
  }

  componentDidMount() {
    var textarea: any = document.getElementById(this.id);
    autosize(textarea);
    this.tribute.attach(textarea);
    textarea.addEventListener('tribute-replaced', () => {
      this.state.siteForm.description = textarea.value;
      this.setState(this.state);
      autosize.update(textarea);
    });
  }

  // Necessary to stop the loading
  componentWillReceiveProps() {
    this.state.loading = false;
    this.setState(this.state);
  }

  render() {
    return (
      <>
        <Prompt
          when={
            !this.state.loading &&
            !this.props.site &&
            Boolean(this.state.siteForm.name || this.state.siteForm.description)
          }
          message={i18n.t('block_leaving')}
        />
        <form onSubmit={this.handleCreateSiteSubmit}>
          <h5>{`${
            this.props.site
              ? capitalizeFirstLetter(i18n.t('edit'))
              : capitalizeFirstLetter(i18n.t('name'))
          } ${i18n.t('your_site')}`}</h5>
          <div className="form-group row">
            <label className="col-12 col-form-label" htmlFor="create-site-name">
              {i18n.t('name')}
            </label>
            <div className="col-12">
              <input
                type="text"
                id="create-site-name"
                className="form-control"
                value={this.state.siteForm.name}
                onInput={this.handleSiteNameChange}
                required
                minLength={3}
                maxLength={20}
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
                value={this.state.siteForm.description}
                onInput={this.handleSiteDescriptionChange}
                className="form-control"
                rows={3}
                maxLength={10000}
              />
            </div>
          </div>
          <div className="form-group row">
            <div className="col-12">
              <div className="form-check">
                <input
                  className="form-check-input"
                  id="create-site-downvotes"
                  type="checkbox"
                  checked={this.state.siteForm.enable_downvotes}
                  onChange={this.handleSiteEnableDownvotesChange}
                />
                <label className="form-check-label" htmlFor="create-site-downvotes">
                  {i18n.t('enable_downvotes')}
                </label>
              </div>
            </div>
          </div>
          <div className="form-group row">
            <div className="col-12">
              <div className="form-check">
                <input
                  className="form-check-input"
                  id="create-site-enable-nsfw"
                  type="checkbox"
                  checked={this.state.siteForm.enable_nsfw}
                  onChange={this.handleSiteEnableNsfwChange}
                />
                <label
                  className="form-check-label"
                  htmlFor="create-site-enable-nsfw"
                >
                  {i18n.t('enable_nsfw')}
                </label>
              </div>
            </div>
          </div>
          <div className="form-group row">
            <div className="col-12">
              <div className="form-check">
                <input
                  className="form-check-input"
                  id="create-site-open-registration"
                  type="checkbox"
                  checked={this.state.siteForm.open_registration}
                  onChange={this.handleSiteOpenRegistrationChange}
                />
                <label
                  className="form-check-label"
                  htmlFor="create-site-open-registration"
                >
                  {i18n.t('open_registration')}
                </label>
              </div>
            </div>
          </div>
          <div className="form-group row">
            <div className="col-12">
              <button type="submit" className="btn btn-secondary mr-2">
                {this.state.loading ? (
                  <svg className="icon icon-spinner spin">
                    <use xlinkHref="#icon-spinner"></use>
                  </svg>
                ) : this.props.site ? (
                  capitalizeFirstLetter(i18n.t('save'))
                ) : (
                  capitalizeFirstLetter(i18n.t('create'))
                )}
              </button>
              {this.props.site && (
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

  handleCreateSiteSubmit = (event: any) => {
    event.preventDefault();
    this.state.loading = true;
    if (this.props.site) {
      WebSocketService.Instance.editSite(this.state.siteForm);
    } else {
      WebSocketService.Instance.createSite(this.state.siteForm);
    }
    this.setState(this.state);
  }

  handleSiteNameChange = (event: any) => {
    this.state.siteForm.name = event.target.value;
    this.setState(this.state);
  }

  handleSiteDescriptionChange = (event: any) => {
    this.state.siteForm.description = event.target.value;
    this.setState(this.state);
  }

  handleSiteEnableNsfwChange = (event: any) => {
    this.state.siteForm.enable_nsfw = event.target.checked;
    this.setState(this.state);
  }

  handleSiteOpenRegistrationChange = (event: any) => {
    this.state.siteForm.open_registration = event.target.checked;
    this.setState(this.state);
  }

  handleSiteEnableDownvotesChange = (event: any) => {
    this.state.siteForm.enable_downvotes = event.target.checked;
    this.setState(this.state);
  }

  handleCancel = () => {
    this.props.onCancel();
  }
}
