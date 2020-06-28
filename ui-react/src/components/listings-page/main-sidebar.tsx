import React, { Component } from 'react';
import { Link } from 'react-router-dom';

import { i18n } from '../../i18next';
import { Trans } from 'react-i18next';

import { repoUrl, mdToHtml } from '../../utils';

// Types/schema
import {
  User,
  Community,
  CommunityUser,
  SortType,
  GetSiteResponse,
  ListingType,
  DataType,
  PostResponse,
  Post,
} from '../../interfaces';

// Components
import { SiteForm } from '../communities/site-form';
import { UserListing } from '../users/user-listing';


interface SidebarProps {
  user: User;
  subscribedCommunities: Array<CommunityUser>;
  trendingCommunities: Array<Community>;
  loading: boolean;
  showEditSite: boolean;
  siteRes: GetSiteResponse;
  canAdmin: () => boolean;
  handleEditCancel: () => void;
  handleEditClick: () => void; 
}

/**
 * Sidebar shown on community/main page.
 */
export class Sidebar extends Component<SidebarProps> {


  /**
   * Sidebar displayed on right hand side.
   */
  render() {
    return (
      <div>
        {!this.props.loading && (
          <div>
            {/* Sidebar: communities */}
            {this.communities()}
            {/* Sidebar: site status */}
            {this.site_status()}
            {/* Sidebar: Welcome message */}
            {this.landing()}
          </div>
        )}
      </div>
    );
  }


  /**
   * Component of sidebar showing communities info.
   */
  communities() {
    return (
      <div className="card border-secondary mb-3">
        <div className="card-body">
          {this.trendingCommunities()}
          {this.props.user &&
            this.props.subscribedCommunities.length > 0 && (
              <div>
                <h5>
                  <Trans i18nKey="subscribed_to_communities">
                    #
                    <Link className="text-body" to="/communities">
                      #
                    </Link>
                  </Trans>
                </h5>
                <ul className="list-inline">
                  {this.props.subscribedCommunities.map(community => (
                    <li className="list-inline-item">
                      <Link to={`/c/${community.community_name}`}>
                        {community.community_name}
                      </Link>
                    </li>
                  ))}
                </ul>
              </div>
            )}
          <Link
            className="btn btn-sm btn-secondary btn-block"
            to="/create_community"
          >
            {i18n.t('create_a_community')}
          </Link>
        </div>
      </div>
    );
  }

  /**
   * Links to trending communities in sidebar.
   */
  trendingCommunities() {
    return (
      <div>
        <h5>
          <Trans i18nKey="trending_communities">
            #
            <Link className="text-body" to="/communities">
              #
            </Link>
          </Trans>
        </h5>
        <ul className="list-inline">
          {this.props.trendingCommunities.map(community => (
            <li className="list-inline-item">
              <Link to={`/c/${community.name}`}>{community.name}</Link>
            </li>
          ))}
        </ul>
      </div>
    );
  }

  /**
   * Component of sidebar showing site info/status.
   */
  site_status() {
    return (
      <div>
        {!this.props.showEditSite ? (
          this.site_info()
        ) : (
          <SiteForm
            site={this.props.siteRes.site}
            onCancel={this.props.handleEditCancel}
          />
        )}
      </div>
    );
  }


  /**
   * Component of sidebar showing introduction message.
   */
  landing() {
    return (
      <div className="card border-secondary">
        <div className="card-body">
          <h5>
            {i18n.t('powered_by')}
            <svg className="icon mx-2">
              <use xlinkHref="#icon-mouse">#</use>
            </svg>
            <a href={repoUrl}>
              Lemmy<sup>beta</sup>
            </a>
          </h5>
          <p className="mb-0">
            <Trans i18nKey="landing_0">
              #
              <a href="https://en.wikipedia.org/wiki/Social_network_aggregation">
                #
              </a>
              <a href="https://en.wikipedia.org/wiki/Fediverse">#</a>
              <br></br>
              <code>#</code>
              <br></br>
              <b>#</b>
              <br></br>
              <a href={repoUrl}>#</a>
              <br></br>
              <a href="https://www.rust-lang.org">#</a>
              <a href="https://actix.rs/">#</a>
              <a href="https://infernojs.org">#</a>
              <a href="https://www.typescriptlang.org/">#</a>
            </Trans>
          </p>
        </div>
      </div>
    );
  }

  /**
   * General info about the site/communities.
   */
  site_info() {
    return (
      <div>
        <div className="card border-secondary mb-3">
          <div className="card-body">
            <h5 className="mb-0">{`${this.props.siteRes.site.name}`}</h5>
            {this.props.canAdmin() && (
              <ul className="list-inline mb-1 text-muted font-weight-bold">
                <li className="list-inline-item-action">
                  <span
                    className="pointer"
                    onClick={this.props.handleEditClick}
                    data-tippy-content={i18n.t('edit')}
                  >
                    <svg className="icon icon-inline">
                      <use xlinkHref="#icon-edit"></use>
                    </svg>
                  </span>
                </li>
              </ul>
            )}
            <ul className="my-2 list-inline">
              {/*
              <li className="list-inline-item badge badge-secondary">
                {i18n.t('number_online', { count: this.props.siteRes.online })}
              </li>
              */}
              <li className="list-inline-item badge badge-secondary">
                {i18n.t('number_of_users', {
                  count: this.props.siteRes.site.number_of_users,
                })}
              </li>
              <li className="list-inline-item badge badge-secondary">
                {i18n.t('number_of_communities', {
                  count: this.props.siteRes.site.number_of_communities,
                })}
              </li>
              <li className="list-inline-item badge badge-secondary">
                {i18n.t('number_of_posts', {
                  count: this.props.siteRes.site.number_of_posts,
                })}
              </li>
              <li className="list-inline-item badge badge-secondary">
                {i18n.t('number_of_comments', {
                  count: this.props.siteRes.site.number_of_comments,
                })}
              </li>
              <li className="list-inline-item">
                <Link className="badge badge-secondary" to="/modlog">
                  {i18n.t('modlog')}
                </Link>
              </li>
            </ul>
            <ul className="mt-1 list-inline small mb-0">
              <li className="list-inline-item">{i18n.t('admins')}:</li>
              {this.props.siteRes.admins.map(admin => (
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
          </div>
        </div>
        {this.props.siteRes.site.description && (
          <div className="card border-secondary mb-3">
            <div className="card-body">
              <div
                className="md-div"
                dangerouslySetInnerHTML={mdToHtml(
                  this.props.siteRes.site.description
                )}
              />
            </div>
          </div>
        )}
      </div>
    );
  }

}