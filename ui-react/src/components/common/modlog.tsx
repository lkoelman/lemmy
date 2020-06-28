import React, { Component } from 'react';
import { Link } from 'react-router-dom';
import { Subscription } from 'rxjs';
import { retryWhen, delay, take } from 'rxjs/operators';
import {
  UserOperation,
  GetModlogForm,
  GetModlogResponse,
  ModRemovePost,
  ModLockPost,
  ModStickyPost,
  ModRemoveComment,
  ModRemoveCommunity,
  ModBanFromCommunity,
  ModBan,
  ModAddCommunity,
  ModAdd,
  WebSocketJsonResponse,
} from '../../interfaces';
import { WebSocketService } from '../../services';
import { wsJsonToRes, addTypeInfo, fetchLimit, toast } from '../../utils';
import { MomentTime } from './moment-time';
import moment from 'moment';
import { i18n } from '../../i18next';

interface ModlogState {
  combined: Array<{
    type_: string;
    data:
      | ModRemovePost
      | ModLockPost
      | ModStickyPost
      | ModRemoveCommunity
      | ModAdd
      | ModBan;
  }>;
  communityId?: number;
  communityName?: string;
  page: number;
  loading: boolean;
}

/**
 * Page showing a log of all moderator actions.
 * 
 * Currently reached via link in site footer.
 */
export class Modlog extends Component<any, ModlogState> {
  private subscription: Subscription;
  private emptyState: ModlogState = {
    combined: [],
    page: 1,
    loading: true,
  };

  constructor(props: any, context: any) {
    super(props, context);

    let state = this.emptyState;
    state.communityId = this.props.match.params.community_id
      ? Number(this.props.match.params.community_id)
      : undefined;
    this.state = state;

    this.subscription = WebSocketService.Instance.subject
      .pipe(retryWhen(errors => errors.pipe(delay(3000), take(10))))
      .subscribe(
        msg => this.parseMessage(msg),
        err => console.error(err),
        () => console.log('complete')
      );

    this.refetch();
  }

  componentWillUnmount() {
    this.subscription.unsubscribe();
  }

  componentDidMount() {
    document.title = `Modlog - ${WebSocketService.Instance.site.name}`;
  }

  setCombined(res: GetModlogResponse) {
    let removed_posts = addTypeInfo(res.removed_posts, 'removed_posts');
    let locked_posts = addTypeInfo(res.locked_posts, 'locked_posts');
    let stickied_posts = addTypeInfo(res.stickied_posts, 'stickied_posts');
    let removed_comments = addTypeInfo(
      res.removed_comments,
      'removed_comments'
    );
    let removed_communities = addTypeInfo(
      res.removed_communities,
      'removed_communities'
    );
    let banned_from_community = addTypeInfo(
      res.banned_from_community,
      'banned_from_community'
    );
    let added_to_community = addTypeInfo(
      res.added_to_community,
      'added_to_community'
    );
    let added = addTypeInfo(res.added, 'added');
    let banned = addTypeInfo(res.banned, 'banned');

    let combined = [];
    combined.push(...removed_posts);
    combined.push(...locked_posts);
    combined.push(...stickied_posts);
    combined.push(...removed_comments);
    combined.push(...removed_communities);
    combined.push(...banned_from_community);
    combined.push(...added_to_community);
    combined.push(...added);
    combined.push(...banned);

    if (this.state.communityId && combined.length > 0) {
      let communityName = (combined[0].data as ModRemovePost).community_name;
      this.setState({ communityName: communityName})
    }

    // Sort them by time
    combined.sort((a, b) =>
      b.data.when_.localeCompare(a.data.when_)
    );

    this.setState({ combined: combined });
  }

  combined() {
    return (
      <tbody>
        {this.state.combined.map(i => (
          <tr>
            <td>
              <MomentTime data={i.data} />
            </td>
            <td>
              <Link to={`/u/${i.data.mod_user_name}`}>
                {i.data.mod_user_name}
              </Link>
            </td>
            <td>
              {i.type_ == 'removed_posts' && (
                <>
                  {(i.data as ModRemovePost).removed ? 'Removed' : 'Restored'}
                  <span>
                    {' '}
                    Post{' '}
                    <Link to={`/post/${(i.data as ModRemovePost).post_id}`}>
                      {(i.data as ModRemovePost).post_name}
                    </Link>
                  </span>
                  <div>
                    {(i.data as ModRemovePost).reason &&
                      ` reason: ${(i.data as ModRemovePost).reason}`}
                  </div>
                </>
              )}
              {i.type_ == 'locked_posts' && (
                <>
                  {(i.data as ModLockPost).locked ? 'Locked' : 'Unlocked'}
                  <span>
                    {' '}
                    Post{' '}
                    <Link to={`/post/${(i.data as ModLockPost).post_id}`}>
                      {(i.data as ModLockPost).post_name}
                    </Link>
                  </span>
                </>
              )}
              {i.type_ == 'stickied_posts' && (
                <>
                  {(i.data as ModStickyPost).stickied
                    ? 'Stickied'
                    : 'Unstickied'}
                  <span>
                    {' '}
                    Post{' '}
                    <Link to={`/post/${(i.data as ModStickyPost).post_id}`}>
                      {(i.data as ModStickyPost).post_name}
                    </Link>
                  </span>
                </>
              )}
              {i.type_ == 'removed_comments' && (
                <>
                  {(i.data as ModRemoveComment).removed
                    ? 'Removed'
                    : 'Restored'}
                  <span>
                    {' '}
                    Comment{' '}
                    <Link
                      to={`/post/${
                        (i.data as ModRemoveComment).post_id
                      }/comment/${(i.data as ModRemoveComment).comment_id}`}
                    >
                      {(i.data as ModRemoveComment).comment_content}
                    </Link>
                  </span>
                  <span>
                    {' '}
                    by{' '}
                    <Link
                      to={`/u/${
                        (i.data as ModRemoveComment).comment_user_name
                      }`}
                    >
                      {(i.data as ModRemoveComment).comment_user_name}
                    </Link>
                  </span>
                  <div>
                    {(i.data as ModRemoveComment).reason &&
                      ` reason: ${(i.data as ModRemoveComment).reason}`}
                  </div>
                </>
              )}
              {i.type_ == 'removed_communities' && (
                <>
                  {(i.data as ModRemoveCommunity).removed
                    ? 'Removed'
                    : 'Restored'}
                  <span>
                    {' '}
                    Community{' '}
                    <Link
                      to={`/c/${(i.data as ModRemoveCommunity).community_name}`}
                    >
                      {(i.data as ModRemoveCommunity).community_name}
                    </Link>
                  </span>
                  <div>
                    {(i.data as ModRemoveCommunity).reason &&
                      ` reason: ${(i.data as ModRemoveCommunity).reason}`}
                  </div>
                  <div>
                    {(i.data as ModRemoveCommunity).expires &&
                      ` expires: ${moment
                        .utc((i.data as ModRemoveCommunity).expires)
                        .fromNow()}`}
                  </div>
                </>
              )}
              {i.type_ == 'banned_from_community' && (
                <>
                  <span>
                    {(i.data as ModBanFromCommunity).banned
                      ? 'Banned '
                      : 'Unbanned '}{' '}
                  </span>
                  <span>
                    <Link
                      to={`/u/${
                        (i.data as ModBanFromCommunity).other_user_name
                      }`}
                    >
                      {(i.data as ModBanFromCommunity).other_user_name}
                    </Link>
                  </span>
                  <span> from the community </span>
                  <span>
                    <Link
                      to={`/c/${
                        (i.data as ModBanFromCommunity).community_name
                      }`}
                    >
                      {(i.data as ModBanFromCommunity).community_name}
                    </Link>
                  </span>
                  <div>
                    {(i.data as ModBanFromCommunity).reason &&
                      ` reason: ${(i.data as ModBanFromCommunity).reason}`}
                  </div>
                  <div>
                    {(i.data as ModBanFromCommunity).expires &&
                      ` expires: ${moment
                        .utc((i.data as ModBanFromCommunity).expires)
                        .fromNow()}`}
                  </div>
                </>
              )}
              {i.type_ == 'added_to_community' && (
                <>
                  <span>
                    {(i.data as ModAddCommunity).removed
                      ? 'Removed '
                      : 'Appointed '}{' '}
                  </span>
                  <span>
                    <Link
                      to={`/u/${(i.data as ModAddCommunity).other_user_name}`}
                    >
                      {(i.data as ModAddCommunity).other_user_name}
                    </Link>
                  </span>
                  <span> as a mod to the community </span>
                  <span>
                    <Link
                      to={`/c/${(i.data as ModAddCommunity).community_name}`}
                    >
                      {(i.data as ModAddCommunity).community_name}
                    </Link>
                  </span>
                </>
              )}
              {i.type_ == 'banned' && (
                <>
                  <span>
                    {(i.data as ModBan).banned ? 'Banned ' : 'Unbanned '}{' '}
                  </span>
                  <span>
                    <Link to={`/u/${(i.data as ModBan).other_user_name}`}>
                      {(i.data as ModBan).other_user_name}
                    </Link>
                  </span>
                  <div>
                    {(i.data as ModBan).reason &&
                      ` reason: ${(i.data as ModBan).reason}`}
                  </div>
                  <div>
                    {(i.data as ModBan).expires &&
                      ` expires: ${moment
                        .utc((i.data as ModBan).expires)
                        .fromNow()}`}
                  </div>
                </>
              )}
              {i.type_ == 'added' && (
                <>
                  <span>
                    {(i.data as ModAdd).removed ? 'Removed ' : 'Appointed '}{' '}
                  </span>
                  <span>
                    <Link to={`/u/${(i.data as ModAdd).other_user_name}`}>
                      {(i.data as ModAdd).other_user_name}
                    </Link>
                  </span>
                  <span> as an admin </span>
                </>
              )}
            </td>
          </tr>
        ))}
      </tbody>
    );
  }

  render() {
    return (
      <div className="container">
        {this.state.loading ? (
          <h5 className="">
            <svg className="icon icon-spinner spin">
              <use xlinkHref="#icon-spinner"></use>
            </svg>
          </h5>
        ) : (
          <div>
            <h5>
              {this.state.communityName && (
                <Link
                  className="text-body"
                  to={`/c/${this.state.communityName}`}
                >
                  /c/{this.state.communityName}{' '}
                </Link>
              )}
              <span>{i18n.t('modlog')}</span>
            </h5>
            <div className="table-responsive">
              <table id="modlog_table" className="table table-sm table-hover">
                <thead className="pointer">
                  <tr>
                    <th> {i18n.t('time')}</th>
                    <th>{i18n.t('mod')}</th>
                    <th>{i18n.t('action')}</th>
                  </tr>
                </thead>
                {this.combined()}
              </table>
              {this.paginator()}
            </div>
          </div>
        )}
      </div>
    );
  }

  paginator() {
    return (
      <div className="mt-2">
        {this.state.page > 1 && (
          <button
            className="btn btn-sm btn-secondary mr-1"
            onClick={this.prevPage}
          >
            {i18n.t('prev')}
          </button>
        )}
        <button
          className="btn btn-sm btn-secondary"
          onClick={this.nextPage}
        >
          {i18n.t('next')}
        </button>
      </div>
    );
  }

  nextPage = () => {
    this.setState((state, props) => {
      return {page: state.page + 1};
    });
    this.refetch();
  }

  prevPage = () => {
    this.setState((state, props) => {
      return {page: state.page - 1};
    });
    this.refetch();
  }

  refetch() {
    let modlogForm: GetModlogForm = {
      community_id: this.state.communityId,
      page: this.state.page,
      limit: fetchLimit,
    };
    WebSocketService.Instance.getModlog(modlogForm);
  }

  parseMessage(msg: WebSocketJsonResponse) {
    console.log(msg);
    let res = wsJsonToRes(msg);
    if (msg.error) {
      toast(i18n.t(msg.error), 'danger');
      return;
    } else if (res.op == UserOperation.GetModlog) {
      let data = res.data as GetModlogResponse;
      this.setState({ loading: false});
      window.scrollTo(0, 0);
      this.setCombined(data);
    }
  }
}
