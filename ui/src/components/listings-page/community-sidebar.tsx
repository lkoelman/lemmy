import React, { Component } from 'react';
import { Link } from 'react-router-dom';
import {
  Community,
  CommunityUser,
  FollowCommunityForm,
  CommunityForm as CommunityFormI,
  UserView,
} from '../../interfaces';
import { WebSocketService, UserService } from '../../services';
import {
  mdToHtml,
  getUnixTime,
  pictshareAvatarThumbnail,
  showAvatars,
} from '../../utils';
import { CommunityForm } from '../communities/community-form';
import { UserListing } from '../users/user-listing';
import { i18n } from '../../i18next';

interface SidebarProps {
  community: Community;
  moderators: Array<CommunityUser>;
  admins: Array<UserView>;
  online: number;
}

interface SidebarState {
  showEdit: boolean;
  showRemoveDialog: boolean;
  removeReason: string;
  removeExpires: string;
}

export class Sidebar extends Component<SidebarProps, SidebarState> {
  private emptyState: SidebarState = {
    showEdit: false,
    showRemoveDialog: false,
    removeReason: null,
    removeExpires: null,
  };

  public state: SidebarState;

  constructor(props: any, context: any) {
    super(props, context);
    this.state = this.emptyState;
    this.handleEditCommunity = this.handleEditCommunity.bind(this);
    this.handleEditCancel = this.handleEditCancel.bind(this);
  }

  render() {
    return (
      <div>
        {!this.state.showEdit ? (
          this.sidebar()
        ) : (
          <CommunityForm
            community={this.props.community}
            onEdit={this.handleEditCommunity}
            onCancel={this.handleEditCancel}
          />
        )}
      </div>
    );
  }

  sidebar() {
    let community = this.props.community;
    return (
      <div>
        <div className="card border-secondary mb-3">
          <div className="card-body">
            <h5 className="mb-0">
              <span>{community.title}</span>
              {community.removed && (
                <small className="ml-2 text-muted font-italic">
                  {i18n.t('removed')}
                </small>
              )}
              {community.deleted && (
                <small className="ml-2 text-muted font-italic">
                  {i18n.t('deleted')}
                </small>
              )}
            </h5>
            <Link className="text-muted" to={`/c/${community.name}`}>
              /c/{community.name}
            </Link>
            <ul className="list-inline mb-1 text-muted font-weight-bold">
              {this.canMod && (
                <>
                  <li className="list-inline-item-action">
                    <span
                      className="pointer"
                      onClick={this.handleEditClick}
                      data-tippy-content={i18n.t('edit')}
                    >
                      <svg className="icon icon-inline">
                        <use xlinkHref="#icon-edit"></use>
                      </svg>
                    </span>
                  </li>
                  {this.amCreator && (
                    <li className="list-inline-item-action">
                      <span
                        className="pointer"
                        onClick={this.handleDeleteClick}
                        data-tippy-content={
                          !community.deleted
                            ? i18n.t('delete')
                            : i18n.t('restore')
                        }
                      >
                        <svg
                          className={`icon icon-inline ${
                            community.deleted && 'text-danger'
                          }`}
                        >
                          <use xlinkHref="#icon-trash"></use>
                        </svg>
                      </span>
                    </li>
                  )}
                </>
              )}
              {this.canAdmin && (
                <li className="list-inline-item">
                  {!this.props.community.removed ? (
                    <span
                      className="pointer"
                      onClick={this.handleModRemoveShow}
                    >
                      {i18n.t('remove')}
                    </span>
                  ) : (
                    <span
                      className="pointer"
                      onClick={this.handleModRemoveSubmit}
                    >
                      {i18n.t('restore')}
                    </span>
                  )}
                </li>
              )}
            </ul>
            {this.state.showRemoveDialog && (
              <form onSubmit={this.handleModRemoveSubmit}>
                <div className="form-group row">
                  <label className="col-form-label" htmlFor="remove-reason">
                    {i18n.t('reason')}
                  </label>
                  <input
                    type="text"
                    id="remove-reason"
                    className="form-control mr-2"
                    placeholder={i18n.t('optional')}
                    value={this.state.removeReason}
                    onInput={this.handleModRemoveReasonChange}
                  />
                </div>
                {/* TODO hold off on expires for now */}
                {/* <div className="form-group row"> */}
                {/*   <label className="col-form-label">Expires</label> */}
                {/*   <input type="date" className="form-control mr-2" placeholder={i18n.t('expires')} value={this.state.removeExpires} onInput={this.handleModRemoveExpiresChange} /> */}
                {/* </div> */}
                <div className="form-group row">
                  <button type="submit" className="btn btn-secondary">
                    {i18n.t('remove_community')}
                  </button>
                </div>
              </form>
            )}
            <ul className="my-1 list-inline">
              {/*
              <li className="list-inline-item badge badge-secondary">
                {i18n.t('number_online', { count: this.props.online })}
              </li>
              */}
              <li className="list-inline-item badge badge-secondary">
                {i18n.t('number_of_subscribers', {
                  count: community.number_of_subscribers,
                })}
              </li>
              <li className="list-inline-item badge badge-secondary">
                {i18n.t('number_of_posts', {
                  count: community.number_of_posts,
                })}
              </li>
              <li className="list-inline-item badge badge-secondary">
                {i18n.t('number_of_comments', {
                  count: community.number_of_comments,
                })}
              </li>
              <li className="list-inline-item">
                <Link className="badge badge-secondary" to="/communities">
                  {community.category_name}
                </Link>
              </li>
              <li className="list-inline-item">
                <Link
                  className="badge badge-secondary"
                  to={`/modlog/community/${this.props.community.id}`}
                >
                  {i18n.t('modlog')}
                </Link>
              </li>
            </ul>
            <ul className="list-inline small">
              <li className="list-inline-item">{i18n.t('mods')}: </li>
              {this.props.moderators.map(mod => (
                <li className="list-inline-item">
                  <UserListing
                    user={{
                      name: mod.user_name,
                      avatar: mod.avatar,
                    }}
                  />
                </li>
              ))}
            </ul>
            <Link
              className={`btn btn-sm btn-secondary btn-block mb-3 ${
                (community.deleted || community.removed) && 'no-click'
              }`}
              to={`/create_post?community=${community.name}`}
            >
              {i18n.t('create_a_post')}
            </Link>
            <div>
              {community.subscribed ? (
                <button
                  className="btn btn-sm btn-secondary btn-block"
                  onClick={(e) => this.handleUnsubscribe(community.id)}
                >
                  {i18n.t('unsubscribe')}
                </button>
              ) : (
                <button
                  className="btn btn-sm btn-secondary btn-block"
                  onClick={(e) => this.handleSubscribe(community.id)}
                >
                  {i18n.t('subscribe')}
                </button>
              )}
            </div>
          </div>
        </div>
        {community.description && (
          <div className="card border-secondary">
            <div className="card-body">
              <div
                className="md-div"
                dangerouslySetInnerHTML={mdToHtml(community.description)}
              />
            </div>
          </div>
        )}
      </div>
    );
  }

  handleEditClick = () => {
    this.state.showEdit = true;
    this.setState(this.state);
  }

  handleEditCommunity() {
    this.state.showEdit = false;
    this.setState(this.state);
  }

  handleEditCancel() {
    this.state.showEdit = false;
    this.setState(this.state);
  }

  handleDeleteClick = (event: any) => {
    event.preventDefault();
    let deleteForm: CommunityFormI = {
      name: this.props.community.name,
      title: this.props.community.title,
      category_id: this.props.community.category_id,
      edit_id: this.props.community.id,
      deleted: !this.props.community.deleted,
      nsfw: this.props.community.nsfw,
      auth: null,
    };
    WebSocketService.Instance.editCommunity(deleteForm);
  }

  handleUnsubscribe = (communityId: number) => {
    let form: FollowCommunityForm = {
      community_id: communityId,
      follow: false,
    };
    WebSocketService.Instance.followCommunity(form);
  }

  handleSubscribe = (communityId: number) => {
    let form: FollowCommunityForm = {
      community_id: communityId,
      follow: true,
    };
    WebSocketService.Instance.followCommunity(form);
  }

  private get amCreator(): boolean {
    return this.props.community.creator_id == UserService.Instance.user.id;
  }

  get canMod(): boolean {
    return (
      UserService.Instance.user &&
      this.props.moderators
        .map(m => m.user_id)
        .includes(UserService.Instance.user.id)
    );
  }

  get canAdmin(): boolean {
    return (
      UserService.Instance.user &&
      this.props.admins.map(a => a.id).includes(UserService.Instance.user.id)
    );
  }

  handleModRemoveShow = () => {
    this.state.showRemoveDialog = true;
    this.setState(this.state);
  }

  handleModRemoveReasonChange = (event: any) => {
    this.state.removeReason = event.target.value;
    this.setState(this.state);
  }

  handleModRemoveExpiresChange = (event: any) => {
    console.log(event.target.value);
    this.state.removeExpires = event.target.value;
    this.setState(this.state);
  }

  handleModRemoveSubmit = (event: any) => {
    event.preventDefault();
    let deleteForm: CommunityFormI = {
      name: this.props.community.name,
      title: this.props.community.title,
      category_id: this.props.community.category_id,
      edit_id: this.props.community.id,
      removed: !this.props.community.removed,
      reason: this.state.removeReason,
      expires: getUnixTime(this.state.removeExpires),
      nsfw: this.props.community.nsfw,
      auth: null,
    };
    WebSocketService.Instance.editCommunity(deleteForm);

    this.state.showRemoveDialog = false;
    this.setState(this.state);
  }
}
