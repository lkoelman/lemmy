/**
 * The listing of a post in a community.
 * 
 * @packageDocumentation
 */
import React, { Component } from 'react';
import { Link } from 'react-router-dom';
import { WebSocketService, UserService } from '../services';
import {
  Post,
  CreatePostLikeForm,
  CreatePostTaggedForm,
  PostForm as PostFormI,
  SavePostForm,
  CommunityUser,
  UserView,
  BanType,
  BanFromCommunityForm,
  BanUserForm,
  AddModToCommunityForm,
  AddAdminForm,
  TransferSiteForm,
  TransferCommunityForm,
} from '../interfaces';

import { MomentTime } from './moment-time';
import { PostForm } from './post-form';
import { IFramelyCard } from './iframely-card';
import { UserListing } from './user-listing';
import {
  md,
  mdToHtml,
  canMod,
  isMod,
  isImage,
  isVideo,
  getUnixTime,
  pictshareImage,
  setupTippy,
  previewLines,
} from '../utils';
import { i18n } from '../i18next';

interface PostListingState {
  showEdit: boolean;
  showRemoveDialog: boolean;
  removeReason: string;
  showBanDialog: boolean;
  banReason: string;
  banExpires: string;
  banType: BanType;
  showConfirmTransferSite: boolean;
  showConfirmTransferCommunity: boolean;
  imageExpanded: boolean;
  viewSource: boolean;
  showAdvanced: boolean;
  my_vote: number;
  my_tags: Array<string>;
  score: number;
  upvotes: number;
  downvotes: number;
}

interface PostListingProps {
  post: Post;
  showCommunity?: boolean;
  showBody?: boolean;
  moderators?: Array<CommunityUser>;
  admins?: Array<UserView>;
}

export class PostListing extends Component<PostListingProps, PostListingState> {
  
  state: PostListingState;

  private emptyState: PostListingState = {
    showEdit: false,
    showRemoveDialog: false,
    removeReason: null,
    showBanDialog: false,
    banReason: null,
    banExpires: null,
    banType: BanType.Community,
    showConfirmTransferSite: false,
    showConfirmTransferCommunity: false,
    imageExpanded: false,
    viewSource: false,
    showAdvanced: false,
    my_vote: this.props.post.my_vote,
    my_tags: this.props.post.my_tags,
    score: this.props.post.score,
    upvotes: this.props.post.upvotes,
    downvotes: this.props.post.downvotes,
  };

  constructor(props: any, context: any) {
    super(props, context);

    this.state = this.emptyState;
  }

  componentWillReceiveProps(nextProps: PostListingProps) {
    this.setState({ my_vote : nextProps.post.my_vote });
    this.setState({ my_tags : nextProps.post.my_tags });
    this.setState({ upvotes : nextProps.post.upvotes });
    this.setState({ downvotes : nextProps.post.downvotes });
    this.setState({ score : nextProps.post.score });
    this.setState(this.state);
  }

  render() {
    return (
      <div className="">
        {!this.state.showEdit ? (
          <>
            {this.listing()}
            {this.body()}
          </>
        ) : (
          <div className="col-12">
            <PostForm
              post={this.props.post}
              onEdit={this.handleEditPost}
              onCancel={this.handleEditCancel}
            />
          </div>
        )}
      </div>
    );
  }

  body() {
    return (
      <div className="row">
        <div className="col-12">
          {this.props.post.url &&
            this.props.showBody &&
            this.props.post.embed_title && (
              <IFramelyCard post={this.props.post} />
            )}
          {this.props.showBody && this.props.post.body && (
            <>
              {this.state.viewSource ? (
                <pre>{this.props.post.body}</pre>
              ) : (
                <div
                  className="md-div"
                  dangerouslySetInnerHTML={mdToHtml(this.props.post.body)}
                />
              )}
            </>
          )}
        </div>
      </div>
    );
  }

  imgThumb(src: string) {
    let post = this.props.post;
    return (
      <img
        className={`img-fluid thumbnail rounded ${
          (post.nsfw || post.community_nsfw) && 'img-blur'
        }`}
        src={src}
      />
    );
  }

  getImage(thumbnail: boolean = false) {
    let post = this.props.post;
    if (isImage(post.url)) {
      if (post.url.includes('pictshare')) {
        return pictshareImage(post.url, thumbnail);
      } else if (post.thumbnail_url) {
        return pictshareImage(post.thumbnail_url, thumbnail);
      } else {
        return post.url;
      }
    } else if (post.thumbnail_url) {
      return pictshareImage(post.thumbnail_url, thumbnail);
    }
  }

  thumbnail() {
    let post = this.props.post;

    if (isImage(post.url)) {
      return (
        <span
          className="text-body pointer"
          data-tippy-content={i18n.t('expand_here')}
          onClick={this.handleImageExpandClick}
        >
          {this.imgThumb(this.getImage(true))}
          <svg className="icon mini-overlay">
            <use xlinkHref="#icon-image"></use>
          </svg>
        </span>
      );
    } else if (post.thumbnail_url) {
      return (
        <a
          className="text-body"
          href={post.url}
          target="_blank"
          title={post.url}
        >
          {this.imgThumb(this.getImage(true))}
          <svg className="icon mini-overlay">
            <use xlinkHref="#icon-external-link"></use>
          </svg>
        </a>
      );
    } else if (post.url) {
      if (isVideo(post.url)) {
        return (
          <div className="embed-responsive embed-responsive-16by9">
            <video
              playsInline
              muted
              loop
              controls
              className="embed-responsive-item"
            >
              <source src={post.url} type="video/mp4" />
            </video>
          </div>
        );
      } else {
        return (
          <a
            className="text-body"
            href={post.url}
            target="_blank"
            title={post.url}
          >
            <svg className="icon thumbnail">
              <use xlinkHref="#icon-external-link"></use>
            </svg>
          </a>
        );
      }
    } else {
      return (
        <Link
          className="text-body"
          to={`/post/${post.id}`}
          title={i18n.t('comments')}
        >
          <svg className="icon thumbnail">
            <use xlinkHref="#icon-message-square"></use>
          </svg>
        </Link>
      );
    }
  }

  listing() {
    let post = this.props.post;
    return (
      <div className="row">
        <div className={`vote-bar col-1 pr-0 small text-center`}>
          
          {/* Upvote button */}
          <button
            className={`btn-animate btn btn-link p-0 ${
              this.state.my_vote == 1 ? 'text-info' : 'text-muted'
            }`}
            // onClick={this.handlePostLike}
            onClick={this.handlePostTagged}
            data-tippy-content={i18n.t('upvote')}
          >
            <svg className="icon upvote">
              <use xlinkHref="#icon-arrow-up1"></use>
            </svg>
          </button>

          {/* Net balance of points (tippy) */}
          <div
            className={`unselectable pointer font-weight-bold text-muted px-1`}
            data-tippy-content={this.pointsTippy}
          >
            {this.state.score}
          </div>
          
          {/* Downvote button (if enabled) */}
          {WebSocketService.Instance.site.enable_downvotes && (
            <button
              className={`btn-animate btn btn-link p-0 ${
                this.state.my_vote == -1 ? 'text-danger' : 'text-muted'
              }`}
              onClick={this.handlePostDisLike}
              data-tippy-content={i18n.t('downvote')}
            >
              <svg className="icon downvote">
                <use xlinkHref="#icon-arrow-down1"></use>
              </svg>
            </button>
          )}

        </div>
        {!this.state.imageExpanded && (
          <div className="col-3 col-sm-2 pr-0 mt-1">
            <div className="position-relative">{this.thumbnail()}</div>
          </div>
        )}
        <div
          className={`${this.state.imageExpanded ? 'col-12' : 'col-8 col-sm-9'}`}
        >
          <div className="row">
            <div className="col-12">
              <div className="post-title">
                <h5 className="mb-0 d-inline">
                  {this.props.showBody && post.url ? (
                    <a
                      className="text-body"
                      href={post.url}
                      target="_blank"
                      title={post.url}
                    >
                      {post.name}
                    </a>
                  ) : (
                    <Link
                      className="text-body"
                      to={`/post/${post.id}`}
                      title={i18n.t('comments')}
                    >
                      {post.name}
                    </Link>
                  )}
                </h5>
                {post.url &&
                  !(new URL(post.url).hostname == window.location.hostname) && (
                    <small className="d-inline-block">
                      <a
                        className="ml-2 text-muted font-italic"
                        href={post.url}
                        target="_blank"
                        title={post.url}
                      >
                        {new URL(post.url).hostname}
                        <svg className="ml-1 icon icon-inline">
                          <use xlinkHref="#icon-external-link"></use>
                        </svg>
                      </a>
                    </small>
                  )}
                {(isImage(post.url) || this.props.post.thumbnail_url) && (
                  <>
                    {!this.state.imageExpanded ? (
                      <span
                        className="text-monospace unselectable pointer ml-2 text-muted small"
                        data-tippy-content={i18n.t('expand_here')}
                        onClick={this.handleImageExpandClick}
                      >
                        <svg className="icon icon-inline">
                          <use xlinkHref="#icon-plus-square"></use>
                        </svg>
                      </span>
                    ) : (
                      <span>
                        <span
                          className="text-monospace unselectable pointer ml-2 text-muted small"
                          onClick={this.handleImageExpandClick}
                        >
                          <svg className="icon icon-inline">
                            <use xlinkHref="#icon-minus-square"></use>
                          </svg>
                        </span>
                        <div>
                          <span
                            className="pointer"
                            onClick={this.handleImageExpandClick}
                          >
                            <img
                              className="img-fluid img-expanded"
                              src={this.getImage()}
                            />
                          </span>
                        </div>
                      </span>
                    )}
                  </>
                )}
                {post.removed && (
                  <small className="ml-2 text-muted font-italic">
                    {i18n.t('removed')}
                  </small>
                )}
                {post.deleted && (
                  <small
                    className="unselectable pointer ml-2 text-muted font-italic"
                    data-tippy-content={i18n.t('deleted')}
                  >
                    <svg className={`icon icon-inline text-danger`}>
                      <use xlinkHref="#icon-trash"></use>
                    </svg>
                  </small>
                )}
                {post.locked && (
                  <small
                    className="unselectable pointer ml-2 text-muted font-italic"
                    data-tippy-content={i18n.t('locked')}
                  >
                    <svg className={`icon icon-inline text-danger`}>
                      <use xlinkHref="#icon-lock"></use>
                    </svg>
                  </small>
                )}
                {post.stickied && (
                  <small
                    className="unselectable pointer ml-2 text-muted font-italic"
                    data-tippy-content={i18n.t('stickied')}
                  >
                    <svg className={`icon icon-inline text-success`}>
                      <use xlinkHref="#icon-pin"></use>
                    </svg>
                  </small>
                )}
                {post.nsfw && (
                  <small className="ml-2 text-muted font-italic">
                    {i18n.t('nsfw')}
                  </small>
                )}
              </div>
            </div>
          </div>
          <div className="row">
            <div className="details col-12">
              <ul className="list-inline mb-0 text-muted small">
                <li className="list-inline-item">
                  <span>{i18n.t('by')} </span>
                  <UserListing
                    user={{
                      name: post.creator_name,
                      avatar: post.creator_avatar,
                    }}
                  />
                  {this.isMod && (
                    <span className="mx-1 badge badge-light">
                      {i18n.t('mod')}
                    </span>
                  )}
                  {this.isAdmin && (
                    <span className="mx-1 badge badge-light">
                      {i18n.t('admin')}
                    </span>
                  )}
                  {(post.banned_from_community || post.banned) && (
                    <span className="mx-1 badge badge-danger">
                      {i18n.t('banned')}
                    </span>
                  )}
                  {this.props.showCommunity && (
                    <span>
                      <span> {i18n.t('to')} </span>
                      <Link to={`/c/${post.community_name}`}>
                        {post.community_name}
                      </Link>
                    </span>
                  )}
                </li>
                <li className="list-inline-item">•</li>
                <li className="list-inline-item">
                  <span>
                    <MomentTime data={post} />
                  </span>
                </li>
                {post.body && (
                  <>
                    <li className="list-inline-item">•</li>
                    <li className="list-inline-item">
                      {/* Using a link with tippy doesn't work on touch devices unfortunately */}
                      <Link
                        className="text-muted"
                        data-tippy-content={md.render(previewLines(post.body))}
                        data-tippy-allowHtml={true}
                        to={`/post/${post.id}`}
                      >
                        <svg className="mr-1 icon icon-inline">
                          <use xlinkHref="#icon-book-open"></use>
                        </svg>
                      </Link>
                    </li>
                  </>
                )}
                <li className="list-inline-item">•</li>
                {this.state.upvotes !== this.state.score && (
                  <>
                    <span
                      className="unselectable pointer mr-2"
                      data-tippy-content={this.pointsTippy}
                    >
                      <li className="list-inline-item">
                        <span className="text-muted">
                          <svg className="small icon icon-inline mr-1">
                            <use xlinkHref="#icon-arrow-up"></use>
                          </svg>
                          {this.state.upvotes}
                        </span>
                      </li>
                      <li className="list-inline-item">
                        <span className="text-muted">
                          <svg className="small icon icon-inline mr-1">
                            <use xlinkHref="#icon-arrow-down"></use>
                          </svg>
                          {this.state.downvotes}
                        </span>
                      </li>
                    </span>
                    <li className="list-inline-item">•</li>
                  </>
                )}
                <li className="list-inline-item">
                  <Link
                    className="text-muted"
                    title={i18n.t('number_of_comments', {
                      count: post.number_of_comments,
                    })}
                    to={`/post/${post.id}`}
                  >
                    <svg className="mr-1 icon icon-inline">
                      <use xlinkHref="#icon-message-square"></use>
                    </svg>
                    {post.number_of_comments}
                  </Link>
                </li>
              </ul>
              {this.props.post.duplicates && (
                <ul className="list-inline mb-1 small text-muted">
                  <>
                    <li className="list-inline-item mr-2">
                      {i18n.t('cross_posted_to')}
                    </li>
                    {this.props.post.duplicates.map(post => (
                      <li className="list-inline-item mr-2">
                        <Link to={`/post/${post.id}`}>
                          {post.community_name}
                        </Link>
                      </li>
                    ))}
                  </>
                </ul>
              )}
              <ul className="list-inline mb-1 text-muted font-weight-bold">
                {UserService.Instance.user && (
                  <>
                    {this.props.showBody && (
                      <>
                        <li className="list-inline-item">
                          <button
                            className="btn btn-sm btn-link btn-animate text-muted"
                            onClick={this.handleSavePostClick}
                            data-tippy-content={
                              post.saved ? i18n.t('unsave') : i18n.t('save')
                            }
                          >
                            <svg
                              className={`icon icon-inline ${
                                post.saved && 'text-warning'
                              }`}
                            >
                              <use xlinkHref="#icon-star"></use>
                            </svg>
                          </button>
                        </li>
                        <li className="list-inline-item">
                          <Link
                            className="btn btn-sm btn-link btn-animate text-muted"
                            to={`/create_post${this.crossPostParams}`}
                            title={i18n.t('cross_post')}
                          >
                            <svg className="icon icon-inline">
                              <use xlinkHref="#icon-copy"></use>
                            </svg>
                          </Link>
                        </li>
                      </>
                    )}
                    {this.myPost && this.props.showBody && (
                      <>
                        <li className="list-inline-item">
                          <button
                            className="btn btn-sm btn-link btn-animate text-muted"
                            onClick={this.handleEditClick}
                            data-tippy-content={i18n.t('edit')}
                          >
                            <svg className="icon icon-inline">
                              <use xlinkHref="#icon-edit"></use>
                            </svg>
                          </button>
                        </li>
                        <li className="list-inline-item">
                          <button
                            className="btn btn-sm btn-link btn-animate text-muted"
                            onClick={this.handleDeleteClick}
                            data-tippy-content={
                              !post.deleted
                                ? i18n.t('delete')
                                : i18n.t('restore')
                            }
                          >
                            <svg
                              className={`icon icon-inline ${
                                post.deleted && 'text-danger'
                              }`}
                            >
                              <use xlinkHref="#icon-trash"></use>
                            </svg>
                          </button>
                        </li>
                      </>
                    )}

                    {!this.state.showAdvanced && this.props.showBody ? (
                      <li className="list-inline-item">
                        <button
                          className="btn btn-sm btn-link btn-animate text-muted"
                          onClick={this.handleShowAdvanced}
                          data-tippy-content={i18n.t('more')}
                        >
                          <svg className="icon icon-inline">
                            <use xlinkHref="#icon-more-vertical"></use>
                          </svg>
                        </button>
                      </li>
                    ) : (
                      <>
                        {this.props.showBody && post.body && (
                          <li className="list-inline-item">
                            <button
                              className="btn btn-sm btn-link btn-animate text-muted"
                              onClick={this.handleViewSource}
                              data-tippy-content={i18n.t('view_source')}
                            >
                              <svg
                                className={`icon icon-inline ${
                                  this.state.viewSource && 'text-success'
                                }`}
                              >
                                <use xlinkHref="#icon-file-text"></use>
                              </svg>
                            </button>
                          </li>
                        )}
                        {this.canModOnSelf && (
                          <>
                            <li className="list-inline-item">
                              <button
                                className="btn btn-sm btn-link btn-animate text-muted"
                                onClick={this.handleModLock}
                                data-tippy-content={
                                  post.locked
                                    ? i18n.t('unlock')
                                    : i18n.t('lock')
                                }
                              >
                                <svg
                                  className={`icon icon-inline ${
                                    post.locked && 'text-danger'
                                  }`}
                                >
                                  <use xlinkHref="#icon-lock"></use>
                                </svg>
                              </button>
                            </li>
                            <li className="list-inline-item">
                              <button
                                className="btn btn-sm btn-link btn-animate text-muted"
                                onClick={this.handleModSticky}
                                data-tippy-content={
                                  post.stickied
                                    ? i18n.t('unsticky')
                                    : i18n.t('sticky')
                                }
                              >
                                <svg
                                  className={`icon icon-inline ${
                                    post.stickied && 'text-success'
                                  }`}
                                >
                                  <use xlinkHref="#icon-pin"></use>
                                </svg>
                              </button>
                            </li>
                          </>
                        )}
                        {/* Mods can ban from community, and appoint as mods to community */}
                        {(this.canMod || this.canAdmin) && (
                          <li className="list-inline-item">
                            {!post.removed ? (
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
                        {this.canMod && (
                          <>
                            {!this.isMod && (
                              <li className="list-inline-item">
                                {!post.banned_from_community ? (
                                  <span
                                    className="pointer"
                                    onClick={this.handleModBanFromCommunityShow}
                                  >
                                    {i18n.t('ban')}
                                  </span>
                                ) : (
                                  <span
                                    className="pointer"
                                    onClick={this.handleModBanFromCommunitySubmit}
                                  >
                                    {i18n.t('unban')}
                                  </span>
                                )}
                              </li>
                            )}
                            {!post.banned_from_community && (
                              <li className="list-inline-item">
                                <span
                                  className="pointer"
                                  onClick={this.handleAddModToCommunity}
                                >
                                  {this.isMod
                                    ? i18n.t('remove_as_mod')
                                    : i18n.t('appoint_as_mod')}
                                </span>
                              </li>
                            )}
                          </>
                        )}
                        {/* Community creators and admins can transfer community to another mod */}
                        {(this.amCommunityCreator || this.canAdmin) &&
                          this.isMod && (
                            <li className="list-inline-item">
                              {!this.state.showConfirmTransferCommunity ? (
                                <span
                                  className="pointer"
                                  onClick={this.handleShowConfirmTransferCommunity}
                                >
                                  {i18n.t('transfer_community')}
                                </span>
                              ) : (
                                <>
                                  <span className="d-inline-block mr-1">
                                    {i18n.t('are_you_sure')}
                                  </span>
                                  <span
                                    className="pointer d-inline-block mr-1"
                                    onClick={this.handleTransferCommunity}
                                  >
                                    {i18n.t('yes')}
                                  </span>
                                  <span
                                    className="pointer d-inline-block"
                                    onClick={this.handleCancelShowConfirmTransferCommunity}
                                  >
                                    {i18n.t('no')}
                                  </span>
                                </>
                              )}
                            </li>
                          )}
                        {/* Admins can ban from all, and appoint other admins */}
                        {this.canAdmin && (
                          <>
                            {!this.isAdmin && (
                              <li className="list-inline-item">
                                {!post.banned ? (
                                  <span
                                    className="pointer"
                                    onClick={this.handleModBanShow}
                                  >
                                    {i18n.t('ban_from_site')}
                                  </span>
                                ) : (
                                  <span
                                    className="pointer"
                                    onClick={this.handleModBanSubmit}
                                  >
                                    {i18n.t('unban_from_site')}
                                  </span>
                                )}
                              </li>
                            )}
                            {!post.banned && (
                              <li className="list-inline-item">
                                <span
                                  className="pointer"
                                  onClick={this.handleAddAdmin}
                                >
                                  {this.isAdmin
                                    ? i18n.t('remove_as_admin')
                                    : i18n.t('appoint_as_admin')}
                                </span>
                              </li>
                            )}
                          </>
                        )}
                        {/* Site Creator can transfer to another admin */}
                        {this.amSiteCreator && this.isAdmin && (
                          <li className="list-inline-item">
                            {!this.state.showConfirmTransferSite ? (
                              <span
                                className="pointer"
                                onClick={this.handleShowConfirmTransferSite}
                              >
                                {i18n.t('transfer_site')}
                              </span>
                            ) : (
                              <>
                                <span className="d-inline-block mr-1">
                                  {i18n.t('are_you_sure')}
                                </span>
                                <span
                                  className="pointer d-inline-block mr-1"
                                  onClick={this.handleTransferSite}
                                >
                                  {i18n.t('yes')}
                                </span>
                                <span
                                  className="pointer d-inline-block"
                                  onClick={this.handleCancelShowConfirmTransferSite}
                                >
                                  {i18n.t('no')}
                                </span>
                              </>
                            )}
                          </li>
                        )}
                      </>
                    )}
                  </>
                )}
              </ul>
              {this.state.showRemoveDialog && (
                <form
                  className="form-inline"
                  onSubmit={this.handleModRemoveSubmit}
                >
                  <input
                    type="text"
                    className="form-control mr-2"
                    placeholder={i18n.t('reason')}
                    value={this.state.removeReason}
                    onInput={this.handleModRemoveReasonChange}
                  />
                  <button type="submit" className="btn btn-secondary">
                    {i18n.t('remove_post')}
                  </button>
                </form>
              )}
              {this.state.showBanDialog && (
                <form onSubmit={this.handleModBanBothSubmit}>
                  <div className="form-group row">
                    <label className="col-form-label" htmlFor="post-listing-reason">
                      {i18n.t('reason')}
                    </label>
                    <input
                      type="text"
                      id="post-listing-reason"
                      className="form-control mr-2"
                      placeholder={i18n.t('reason')}
                      value={this.state.banReason}
                      onInput={this.handleModBanReasonChange}
                    />
                  </div>
                  {/* TODO hold off on expires until later */}
                  {/* <div className="form-group row"> */}
                  {/*   <label className="col-form-label">Expires</label> */}
                  {/*   <input type="date" className="form-control mr-2" placeholder={i18n.t('expires')} value={this.state.banExpires} onInput={this.handleModBanExpiresChange} /> */}
                  {/* </div> */}
                  <div className="form-group row">
                    <button type="submit" className="btn btn-secondary">
                      {i18n.t('ban')} {post.creator_name}
                    </button>
                  </div>
                </form>
              )}
            </div>
          </div>
        </div>
      </div>
    );
  }

  private get myPost(): boolean {
    return (
      UserService.Instance.user &&
      this.props.post.creator_id == UserService.Instance.user.id
    );
  }

  get isMod(): boolean {
    return (
      this.props.moderators &&
      isMod(
        this.props.moderators.map(m => m.user_id),
        this.props.post.creator_id
      )
    );
  }

  get isAdmin(): boolean {
    return (
      this.props.admins &&
      isMod(
        this.props.admins.map(a => a.id),
        this.props.post.creator_id
      )
    );
  }

  get canMod(): boolean {
    if (this.props.admins && this.props.moderators) {
      let adminsThenMods = this.props.admins
        .map(a => a.id)
        .concat(this.props.moderators.map(m => m.user_id));

      return canMod(
        UserService.Instance.user,
        adminsThenMods,
        this.props.post.creator_id
      );
    } else {
      return false;
    }
  }

  get canModOnSelf(): boolean {
    if (this.props.admins && this.props.moderators) {
      let adminsThenMods = this.props.admins
        .map(a => a.id)
        .concat(this.props.moderators.map(m => m.user_id));

      return canMod(
        UserService.Instance.user,
        adminsThenMods,
        this.props.post.creator_id,
        true
      );
    } else {
      return false;
    }
  }

  get canAdmin(): boolean {
    return (
      this.props.admins &&
      canMod(
        UserService.Instance.user,
        this.props.admins.map(a => a.id),
        this.props.post.creator_id
      )
    );
  }

  get amCommunityCreator(): boolean {
    return (
      this.props.moderators &&
      UserService.Instance.user &&
      this.props.post.creator_id != UserService.Instance.user.id &&
      UserService.Instance.user.id == this.props.moderators[0].user_id
    );
  }

  get amSiteCreator(): boolean {
    return (
      this.props.admins &&
      UserService.Instance.user &&
      this.props.post.creator_id != UserService.Instance.user.id &&
      UserService.Instance.user.id == this.props.admins[0].id
    );
  }

  /**
   * Handle tags added by user.
   * 
   * @param i PostListing that received tags
   */
  handlePostTagged = () => {

    // FIXME: remove placeholder tags
    this.state.my_tags = ["like", "informative", "science"];

    let form: CreatePostTaggedForm = {
      post_id: this.props.post.id,
      tags: this.state.my_tags,
    };

    console.log("Sending tags over websocket.");
    WebSocketService.Instance.tagPost(form);
    console.log("Sent tags over socker.")
  }

  handlePostLike = () => {
    let new_vote = this.state.my_vote == 1 ? 0 : 1;

    if (this.state.my_vote == 1) {
      this.state.score--;
      this.state.upvotes--;
    } else if (this.state.my_vote == -1) {
      this.state.downvotes--;
      this.state.upvotes++;
      this.state.score += 2;
    } else {
      this.state.upvotes++;
      this.state.score++;
    }

    this.setState({ my_vote : new_vote });

    let form: CreatePostLikeForm = {
      post_id: this.props.post.id,
      score: this.state.my_vote,
    };

    WebSocketService.Instance.likePost(form);
    this.setState(this.state);
    setupTippy();
  }

  handlePostDisLike = () => {
    let new_vote = this.state.my_vote == -1 ? 0 : -1;

    if (this.state.my_vote == 1) {
      this.state.score -= 2;
      this.state.upvotes--;
      this.state.downvotes++;
    } else if (this.state.my_vote == -1) {
      this.state.downvotes--;
      this.state.score++;
    } else {
      this.state.downvotes++;
      this.state.score--;
    }

    this.setState({ my_vote : new_vote });

    let form: CreatePostLikeForm = {
      post_id: this.props.post.id,
      score: this.state.my_vote,
    };

    WebSocketService.Instance.likePost(form);
    this.setState(this.state);
    setupTippy();
  }

  handleEditClick = () => {
    this.setState({ showEdit : true });
    this.setState(this.state);
  }

  handleEditCancel = () => {
    this.setState({ showEdit : false });
    this.setState(this.state);
  }

  // The actual editing is done in the recieve for post
  handleEditPost = () => {
    this.setState({ showEdit : false });
    this.setState(this.state);
  }

  handleDeleteClick = () => {
    let deleteForm: PostFormI = {
      body: this.props.post.body,
      community_id: this.props.post.community_id,
      name: this.props.post.name,
      url: this.props.post.url,
      edit_id: this.props.post.id,
      creator_id: this.props.post.creator_id,
      deleted: !this.props.post.deleted,
      nsfw: this.props.post.nsfw,
      auth: null,
    };
    WebSocketService.Instance.editPost(deleteForm);
  }

  handleSavePostClick = () => {
    let saved = this.props.post.saved == undefined ? true : !this.props.post.saved;
    let form: SavePostForm = {
      post_id: this.props.post.id,
      save: saved,
    };

    WebSocketService.Instance.savePost(form);
  }

  get crossPostParams(): string {
    let params = `?title=${this.props.post.name}`;
    let post = this.props.post;

    if (post.url) {
      params += `&url=${post.url}`;
    }
    if (this.props.post.body) {
      params += `&body=${this.props.post.body}`;
    }
    return params;
  }

  handleModRemoveShow = () => {
    this.setState({ showRemoveDialog : true });
    this.setState(this.state);
  }

  handleModRemoveReasonChange = (event: any) => {
    this.setState({ removeReason : event.target.value });
    this.setState(this.state);
  }

  handleModRemoveSubmit = (event: any) => {
    event.preventDefault();
    let form: PostFormI = {
      name: this.props.post.name,
      community_id: this.props.post.community_id,
      edit_id: this.props.post.id,
      creator_id: this.props.post.creator_id,
      removed: !this.props.post.removed,
      reason: this.state.removeReason,
      nsfw: this.props.post.nsfw,
      auth: null,
    };
    WebSocketService.Instance.editPost(form);

    this.setState({ showRemoveDialog : false });
    this.setState(this.state);
  }

  handleModLock = () => {
    let form: PostFormI = {
      name: this.props.post.name,
      community_id: this.props.post.community_id,
      edit_id: this.props.post.id,
      creator_id: this.props.post.creator_id,
      nsfw: this.props.post.nsfw,
      locked: !this.props.post.locked,
      auth: null,
    };
    WebSocketService.Instance.editPost(form);
  }

  handleModSticky = () => {
    let form: PostFormI = {
      name: this.props.post.name,
      community_id: this.props.post.community_id,
      edit_id: this.props.post.id,
      creator_id: this.props.post.creator_id,
      nsfw: this.props.post.nsfw,
      stickied: !this.props.post.stickied,
      auth: null,
    };
    WebSocketService.Instance.editPost(form);
  }

  handleModBanFromCommunityShow = () => {
    this.setState({ showBanDialog : true });
    this.setState({ banType : BanType.Community });
    this.setState(this.state);
  }

  handleModBanShow = () => {
    this.setState({ showBanDialog : true });
    this.setState({ banType : BanType.Site });
    this.setState(this.state);
  }

  handleModBanReasonChange = (event: any) => {
    this.setState({ banReason : event.target.value });
    this.setState(this.state);
  }

  handleModBanExpiresChange = (event: any) => {
    this.setState({ banExpires : event.target.value });
    this.setState(this.state);
  }

  handleModBanFromCommunitySubmit = () => {
    this.setState({ banType : BanType.Community });
    this.setState(this.state);
    this.handleModBanBothSubmit(this);
  }

  handleModBanSubmit = () => {
    this.setState({ banType : BanType.Site });
    this.setState(this.state);
    this.handleModBanBothSubmit(this);
  }

  handleModBanBothSubmit = (event: any) => {
    event.preventDefault();

    if (this.state.banType == BanType.Community) {
      let form: BanFromCommunityForm = {
        user_id: this.props.post.creator_id,
        community_id: this.props.post.community_id,
        ban: !this.props.post.banned_from_community,
        reason: this.state.banReason,
        expires: getUnixTime(this.state.banExpires),
      };
      WebSocketService.Instance.banFromCommunity(form);
    } else {
      let form: BanUserForm = {
        user_id: this.props.post.creator_id,
        ban: !this.props.post.banned,
        reason: this.state.banReason,
        expires: getUnixTime(this.state.banExpires),
      };
      WebSocketService.Instance.banUser(form);
    }

    this.setState({ showBanDialog : false });
    this.setState(this.state);
  }

  handleAddModToCommunity = () => {
    let form: AddModToCommunityForm = {
      user_id: this.props.post.creator_id,
      community_id: this.props.post.community_id,
      added: !this.isMod,
    };
    WebSocketService.Instance.addModToCommunity(form);
    this.setState(this.state);
  }

  handleAddAdmin = () => {
    let form: AddAdminForm = {
      user_id: this.props.post.creator_id,
      added: !this.isAdmin,
    };
    WebSocketService.Instance.addAdmin(form);
    this.setState(this.state);
  }

  handleShowConfirmTransferCommunity = () => {
    this.setState({ showConfirmTransferCommunity : true });
    this.setState(this.state);
  }

  handleCancelShowConfirmTransferCommunity = () => {
    this.setState({ showConfirmTransferCommunity : false });
    this.setState(this.state);
  }

  handleTransferCommunity = () => {
    let form: TransferCommunityForm = {
      community_id: this.props.post.community_id,
      user_id: this.props.post.creator_id,
    };
    WebSocketService.Instance.transferCommunity(form);
    this.setState({ showConfirmTransferCommunity : false });
    this.setState(this.state);
  }

  handleShowConfirmTransferSite = () => {
    this.setState({ showConfirmTransferSite : true });
    this.setState(this.state);
  }

  handleCancelShowConfirmTransferSite = () => {
    this.setState({ showConfirmTransferSite : false });
    this.setState(this.state);
  }

  handleTransferSite = () => {
    let form: TransferSiteForm = {
      user_id: this.props.post.creator_id,
    };
    WebSocketService.Instance.transferSite(form);
    this.setState({ showConfirmTransferSite : false });
    this.setState(this.state);
  }

  handleImageExpandClick = () => {
    this.state.imageExpanded = !this.state.imageExpanded;
    this.setState(this.state);
  }

  handleViewSource = () => {
    this.state.viewSource = !this.state.viewSource;
    this.setState(this.state);
  }

  handleShowAdvanced = () => {
    this.state.showAdvanced = !this.state.showAdvanced;
    this.setState(this.state);
    setupTippy();
  }

  get pointsTippy(): string {
    let points = i18n.t('number_of_points', {
      count: this.state.score,
    });

    let upvotes = i18n.t('number_of_upvotes', {
      count: this.state.upvotes,
    });

    let downvotes = i18n.t('number_of_downvotes', {
      count: this.state.downvotes,
    });

    return `${points} • ${upvotes} • ${downvotes}`;
  }
}
