import React, { Component } from 'react';
import { Subscription } from 'rxjs';
import { retryWhen, delay, take } from 'rxjs/operators';
import {
  UserOperation,
  SiteResponse,
  GetSiteResponse,
  SiteConfigForm,
  GetSiteConfigResponse,
  WebSocketJsonResponse,
} from '../interfaces';
import { WebSocketService } from '../services';
import { wsJsonToRes, capitalizeFirstLetter, toast, randomStr } from '../utils';
import autosize from 'autosize';
import { SiteForm } from './site-form';
import { UserListing } from './user-listing';
import { i18n } from '../i18next';

interface AdminSettingsState {
  siteRes: GetSiteResponse;
  siteConfigRes: GetSiteConfigResponse;
  siteConfigForm: SiteConfigForm;
  loading: boolean;
  siteConfigLoading: boolean;
}

export class AdminSettings extends Component<any, AdminSettingsState> {
  private siteConfigTextAreaId = `site-config-${randomStr()}`;
  private subscription: Subscription;
  private emptyState: AdminSettingsState = {
    siteRes: {
      site: {
        id: null,
        name: null,
        creator_id: null,
        creator_name: null,
        published: null,
        number_of_users: null,
        number_of_posts: null,
        number_of_comments: null,
        number_of_communities: null,
        enable_downvotes: null,
        open_registration: null,
        enable_nsfw: null,
      },
      admins: [],
      banned: [],
      online: null,
    },
    siteConfigForm: {
      config_hjson: null,
      auth: null,
    },
    siteConfigRes: {
      config_hjson: null,
    },
    loading: true,
    siteConfigLoading: null,
  };

  constructor(props: any, context: any) {
    super(props, context);

    this.state = this.emptyState;

    this.subscription = WebSocketService.Instance.subject
      .pipe(retryWhen(errors => errors.pipe(delay(3000), take(10))))
      .subscribe(
        msg => this.parseMessage(msg),
        err => console.error(err),
        () => console.log('complete')
      );

    WebSocketService.Instance.getSite();
    WebSocketService.Instance.getSiteConfig();
  }

  componentWillUnmount() {
    this.subscription.unsubscribe();
  }

  render() {
    return (
      <div className="container">
        {this.state.loading ? (
          <h5>
            <svg className="icon icon-spinner spin">
              <use xlinkHref="#icon-spinner"></use>
            </svg>
          </h5>
        ) : (
          <div className="row">
            <div className="col-12 col-md-6">
              <SiteForm site={this.state.siteRes.site} />
              {this.admins()}
              {this.bannedUsers()}
            </div>
            <div className="col-12 col-md-6">{this.adminSettings()}</div>
          </div>
        )}
      </div>
    );
  }

  admins() {
    return (
      <>
        <h5>{capitalizeFirstLetter(i18n.t('admins'))}</h5>
        <ul className="list-unstyled">
          {this.state.siteRes.admins.map(admin => (
            <li className="list-inline-item">
              <UserListing
                user={{
                  name: admin.name,
                  avatar: admin.avatar,
                }}
              />
            </li>
          ))}
        </ul>
      </>
    );
  }

  bannedUsers() {
    return (
      <>
        <h5>{i18n.t('banned_users')}</h5>
        <ul className="list-unstyled">
          {this.state.siteRes.banned.map(banned => (
            <li className="list-inline-item">
              <UserListing
                user={{
                  name: banned.name,
                  avatar: banned.avatar,
                }}
              />
            </li>
          ))}
        </ul>
      </>
    );
  }

  adminSettings() {
    return (
      <div>
        <h5>{i18n.t('admin_settings')}</h5>
        <form onSubmit={this.handleSiteConfigSubmit}>
          <div className="form-group row">
            <label
              className="col-12 col-form-label"
              htmlFor={this.siteConfigTextAreaId}
            >
              {i18n.t('site_config')}
            </label>
            <div className="col-12">
              <textarea
                id={this.siteConfigTextAreaId}
                value={this.state.siteConfigForm.config_hjson}
                onInput={this.handleSiteConfigHjsonChange}
                className="form-control text-monospace"
                rows={3}
              />
            </div>
          </div>
          <div className="form-group row">
            <div className="col-12">
              <button type="submit" className="btn btn-secondary mr-2">
                {this.state.siteConfigLoading ? (
                  <svg className="icon icon-spinner spin">
                    <use xlinkHref="#icon-spinner"></use>
                  </svg>
                ) : (
                  capitalizeFirstLetter(i18n.t('save'))
                )}
              </button>
            </div>
          </div>
        </form>
      </div>
    );
  }

  handleSiteConfigSubmit = (event: any) => {
    event.preventDefault();
    this.setState({ siteConfigLoading : true });
    WebSocketService.Instance.saveSiteConfig(this.state.siteConfigForm);
  }

  handleSiteConfigHjsonChange = (event: any) => {
    this.state.siteConfigForm.config_hjson = event.target.value;
  }

  parseMessage(msg: WebSocketJsonResponse) {
    console.log(msg);
    let res = wsJsonToRes(msg);
    if (msg.error) {
      toast(i18n.t(msg.error), 'danger');
      this.context.router.history.push('/');
      this.setState({ loading : false });
      return;
    } else if (msg.reconnect) {
    } else if (res.op == UserOperation.GetSite) {
      let data = res.data as GetSiteResponse;

      // This means it hasn't been set up yet
      if (!data.site) {
        this.context.router.history.push('/setup');
      }
      this.setState({ siteRes : data });
      document.title = `${i18n.t('admin_settings')} - ${
        this.state.siteRes.site.name
      }`;
    } else if (res.op == UserOperation.EditSite) {
      let data = res.data as SiteResponse;
      this.setState({ siteRes: { ...this.state.siteRes, site: data.site}});
      toast(i18n.t('site_saved'));
    } else if (res.op == UserOperation.GetSiteConfig) {
      let data = res.data as GetSiteConfigResponse;
      this.setState({ siteConfigRes : data });
      this.setState({ loading : false });
      this.setState({
        siteConfigForm: {
          ...this.state.siteConfigForm,
          config_hjson: this.state.siteConfigRes.config_hjson
        }
      });
      var textarea: any = document.getElementById(this.siteConfigTextAreaId);
      autosize(textarea);
    } else if (res.op == UserOperation.SaveSiteConfig) {
      let data = res.data as GetSiteConfigResponse;
      this.setState({ siteConfigRes : data });
      this.setState({
        siteConfigForm: {
          ...this.state.siteConfigForm,
          config_hjson: this.state.siteConfigRes.config_hjson
        }
      });
      this.setState({ siteConfigLoading : false });
      toast(i18n.t('site_saved'));
    }
  }
}
