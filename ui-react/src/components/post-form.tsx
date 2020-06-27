import React, { Component } from 'react';
import { Prompt } from 'react-router-dom';
import { PostListings } from './post-listings';
import { Subscription } from 'rxjs';
import { retryWhen, delay, take } from 'rxjs/operators';
import {
  PostForm as PostFormI,
  PostFormParams,
  Post,
  PostResponse,
  UserOperation,
  Community,
  ListCommunitiesResponse,
  ListCommunitiesForm,
  SortType,
  SearchForm,
  SearchType,
  SearchResponse,
  GetSiteResponse,
  WebSocketJsonResponse,
} from '../interfaces';
import { WebSocketService, UserService } from '../services';
import {
  wsJsonToRes,
  getPageTitle,
  validURL,
  capitalizeFirstLetter,
  markdownHelpUrl,
  archiveUrl,
  mdToHtml,
  debounce,
  isImage,
  toast,
  randomStr,
  setupTribute,
  setupTippy,
  emojiPicker,
} from '../utils';
import autosize from 'autosize';
import Tribute from 'tributejs/src/Tribute.js';
import emojiShortName from 'emoji-short-name';
import Selectr from 'mobius1-selectr';
import { i18n } from '../i18next';

const MAX_POST_TITLE_LENGTH = 200;

interface PostFormProps {
  post?: Post; // If a post is given, that means this is an edit
  params?: PostFormParams;
  onCancel?(): any;
  onCreate?(id: number): any;
  onEdit?(post: Post): any;
}

interface PostFormState {
  postForm: PostFormI;
  communities: Array<Community>;
  loading: boolean;
  imageLoading: boolean;
  previewMode: boolean;
  suggestedTitle: string;
  suggestedPosts: Array<Post>;
  crossPosts: Array<Post>;
  enable_nsfw: boolean;
}

export class PostForm extends Component<PostFormProps, PostFormState> {
  private id = `post-form-${randomStr()}`;
  private tribute: Tribute;
  private subscription: Subscription;

  state : PostFormState;

  private emptyState: PostFormState = {
    postForm: {
      name: null,
      nsfw: false,
      auth: null,
      community_id: null,
      creator_id: UserService.Instance.user
        ? UserService.Instance.user.id
        : null,
    },
    communities: [],
    loading: false,
    imageLoading: false,
    previewMode: false,
    suggestedTitle: undefined,
    suggestedPosts: [],
    crossPosts: [],
    enable_nsfw: undefined,
  };

  constructor(props: any, context: any) {
    super(props, context);
    this.fetchSimilarPosts = debounce(this.fetchSimilarPosts).bind(this);
    this.fetchPageTitle = debounce(this.fetchPageTitle).bind(this);

    this.tribute = setupTribute();
    this.setupEmojiPicker();

    this.state = this.emptyState;

    if (this.props.post) {
      this.state.postForm = {
        body: this.props.post.body,
        // NOTE: debouncing breaks both these for some reason, unless you use defaultValue
        name: this.props.post.name,
        community_id: this.props.post.community_id,
        edit_id: this.props.post.id,
        creator_id: this.props.post.creator_id,
        url: this.props.post.url,
        nsfw: this.props.post.nsfw,
        auth: null,
      };
    }

    if (this.props.params) {
      this.state.postForm.name = this.props.params.name;
      if (this.props.params.url) {
        this.state.postForm.url = this.props.params.url;
      }
      if (this.props.params.body) {
        this.state.postForm.body = this.props.params.body;
      }
    }

    this.subscription = WebSocketService.Instance.subject
      .pipe(retryWhen(errors => errors.pipe(delay(3000), take(10))))
      .subscribe(
        msg => this.parseMessage(msg),
        err => console.error(err),
        () => console.log('complete')
      );

    let listCommunitiesForm: ListCommunitiesForm = {
      sort: SortType[SortType.TopAll],
      limit: 9999,
    };

    WebSocketService.Instance.listCommunities(listCommunitiesForm);
    WebSocketService.Instance.getSite();
  }

  componentDidMount() {
    var textarea: any = document.getElementById(this.id);
    autosize(textarea);
    this.tribute.attach(textarea);
    textarea.addEventListener('tribute-replaced', () => {
      this.state.postForm.body = textarea.value;
      this.setState(this.state);
      autosize.update(textarea);
    });
    setupTippy();
  }

  componentWillUnmount() {
    this.subscription.unsubscribe();
  }

  render() {
    return (
      <div>
        <Prompt
          when={
            (!this.state.loading &&
              Boolean(this.state.postForm.name ||
                this.state.postForm.url ||
                this.state.postForm.body))
          }
          message={i18n.t('block_leaving')}
        />
        <form onSubmit={this.handlePostSubmit}>
          <div className="form-group row">
            <label className="col-sm-2 col-form-label" htmlFor="post-url">
              {i18n.t('url')}
            </label>
            <div className="col-sm-10">
              <input
                type="url"
                id="post-url"
                className="form-control"
                value={this.state.postForm.url}
                onInput={this.handlePostUrlChange}
                onPaste={this.handleImageUploadPaste}
              />
              {this.state.suggestedTitle && (
                <div
                  className="mt-1 text-muted small font-weight-bold pointer"
                  onClick={this.copySuggestedTitle}
                >
                  {i18n.t('copy_suggested_title', {
                    title: this.state.suggestedTitle,
                  })}
                </div>
              )}
              <form>
                <label
                  htmlFor="file-upload"
                  className={`${
                    UserService.Instance.user && 'pointer'
                  } d-inline-block float-right text-muted font-weight-bold`}
                  data-tippy-content={i18n.t('upload_image')}
                >
                  <svg className="icon icon-inline">
                    <use xlinkHref="#icon-image"></use>
                  </svg>
                </label>
                <input
                  id="file-upload"
                  type="file"
                  accept="image/*,video/*"
                  name="file"
                  className="d-none"
                  disabled={!UserService.Instance.user}
                  onChange={this.handleImageUpload}
                />
              </form>
              {validURL(this.state.postForm.url) && (
                <a
                  href={`${archiveUrl}/?run=1&url=${encodeURIComponent(
                    this.state.postForm.url
                  )}`}
                  target="_blank"
                  className="mr-2 d-inline-block float-right text-muted small font-weight-bold"
                >
                  {i18n.t('archive_link')}
                </a>
              )}
              {this.state.imageLoading && (
                <svg className="icon icon-spinner spin">
                  <use xlinkHref="#icon-spinner"></use>
                </svg>
              )}
              {isImage(this.state.postForm.url) && (
                <img src={this.state.postForm.url} className="img-fluid" />
              )}
              {this.state.crossPosts.length > 0 && (
                <>
                  <div className="my-1 text-muted small font-weight-bold">
                    {i18n.t('cross_posts')}
                  </div>
                  <PostListings showCommunity posts={this.state.crossPosts} />
                </>
              )}
            </div>
          </div>
          <div className="form-group row">
            <label className="col-sm-2 col-form-label" htmlFor="post-title">
              {i18n.t('title')}
            </label>
            <div className="col-sm-10">
              <textarea
                value={this.state.postForm.name}
                id="post-title"
                onInput={this.handlePostNameChange}
                className="form-control"
                required
                rows={2}
                minLength={3}
                maxLength={MAX_POST_TITLE_LENGTH}
              />
              {this.state.suggestedPosts.length > 0 && (
                <>
                  <div className="my-1 text-muted small font-weight-bold">
                    {i18n.t('related_posts')}
                  </div>
                  <PostListings posts={this.state.suggestedPosts} />
                </>
              )}
            </div>
          </div>

          <div className="form-group row">
            <label className="col-sm-2 col-form-label" htmlFor={this.id}>
              {i18n.t('body')}
            </label>
            <div className="col-sm-10">
              <textarea
                id={this.id}
                value={this.state.postForm.body}
                onInput={this.handlePostBodyChange}
                className={`form-control ${this.state.previewMode && 'd-none'}`}
                rows={4}
                maxLength={10000}
              />
              {this.state.previewMode && (
                <div
                  className="md-div"
                  dangerouslySetInnerHTML={mdToHtml(this.state.postForm.body)}
                />
              )}
              {this.state.postForm.body && (
                <button
                  className={`mt-1 mr-2 btn btn-sm btn-secondary ${
                    this.state.previewMode && 'active'
                  }`}
                  onClick={this.handlePreviewToggle}
                >
                  {i18n.t('preview')}
                </button>
              )}
              <a
                href={markdownHelpUrl}
                target="_blank"
                className="d-inline-block float-right text-muted font-weight-bold"
                title={i18n.t('formatting_help')}
              >
                <svg className="icon icon-inline">
                  <use xlinkHref="#icon-help-circle"></use>
                </svg>
              </a>
              <span
                onClick={this.handleEmojiPickerClick}
                className="pointer unselectable d-inline-block mr-3 float-right text-muted font-weight-bold"
                data-tippy-content={i18n.t('emoji_picker')}
              >
                <svg className="icon icon-inline">
                  <use xlinkHref="#icon-smile"></use>
                </svg>
              </span>
            </div>
          </div>
          {!this.props.post && (
            <div className="form-group row">
              <label className="col-sm-2 col-form-label" htmlFor="post-community">
                {i18n.t('community')}
              </label>
              <div className="col-sm-10">
                <select
                  className="form-control"
                  id="post-community"
                  value={this.state.postForm.community_id}
                  onInput={this.handlePostCommunityChange}
                >
                  <option>{i18n.t('select_a_community')}</option>
                  {this.state.communities.map(community => (
                    <option value={community.id}>{community.name}</option>
                  ))}
                </select>
              </div>
            </div>
          )}
          {this.state.enable_nsfw && (
            <div className="form-group row">
              <div className="col-sm-10">
                <div className="form-check">
                  <input
                    className="form-check-input"
                    id="post-nsfw"
                    type="checkbox"
                    checked={this.state.postForm.nsfw}
                    onChange={this.handlePostNsfwChange}
                  />
                  <label className="form-check-label" htmlFor="post-nsfw">
                    {i18n.t('nsfw')}
                  </label>
                </div>
              </div>
            </div>
          )}
          <div className="form-group row">
            <div className="col-sm-10">
              <button
                disabled={!this.state.postForm.community_id}
                type="submit"
                className="btn btn-secondary mr-2"
              >
                {this.state.loading ? (
                  <svg className="icon icon-spinner spin">
                    <use xlinkHref="#icon-spinner"></use>
                  </svg>
                ) : this.props.post ? (
                  capitalizeFirstLetter(i18n.t('save'))
                ) : (
                  capitalizeFirstLetter(i18n.t('create'))
                )}
              </button>
              {this.props.post && (
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
      </div>
    );
  }

  setupEmojiPicker() {
    emojiPicker.on('emoji', twemojiHtmlStr => {
      if (this.state.postForm.body == null) {
        this.state.postForm.body = '';
      }
      var el = document.createElement('div');
      el.innerHTML = twemojiHtmlStr;
      let nativeUnicode = (el.childNodes[0] as HTMLElement).getAttribute('alt');
      let shortName = `:${emojiShortName[nativeUnicode]}:`;
      this.state.postForm.body += shortName;
      this.setState(this.state);
    });
  }

  handlePostSubmit = (event: any) => {
    event.preventDefault();
    if (this.props.post) {
      WebSocketService.Instance.editPost(this.state.postForm);
    } else {
      WebSocketService.Instance.createPost(this.state.postForm);
    }
    this.state.loading = true;
    this.setState(this.state);
  }

  copySuggestedTitle = () => {
    this.state.postForm.name = this.state.suggestedTitle.substring(
      0,
      MAX_POST_TITLE_LENGTH
    );
    this.state.suggestedTitle = undefined;
    this.setState(this.state);
  }

  handlePostUrlChange = (event: any) => {
    this.state.postForm.url = event.target.value;
    this.setState(this.state);
    this.fetchPageTitle();
  }

  fetchPageTitle() {
    if (validURL(this.state.postForm.url)) {
      let form: SearchForm = {
        q: this.state.postForm.url,
        type_: SearchType[SearchType.Url],
        sort: SortType[SortType.TopAll],
        page: 1,
        limit: 6,
      };

      WebSocketService.Instance.search(form);

      // Fetch the page title
      getPageTitle(this.state.postForm.url).then(d => {
        this.state.suggestedTitle = d;
        this.setState(this.state);
      });
    } else {
      this.state.suggestedTitle = undefined;
      this.state.crossPosts = [];
    }
  }

  handlePostNameChange = (event: any) => {
    this.state.postForm.name = event.target.value;
    this.setState(this.state);
    this.fetchSimilarPosts();
  }

  fetchSimilarPosts() {
    let form: SearchForm = {
      q: this.state.postForm.name,
      type_: SearchType[SearchType.Posts],
      sort: SortType[SortType.TopAll],
      community_id: this.state.postForm.community_id,
      page: 1,
      limit: 6,
    };

    if (this.state.postForm.name !== '') {
      WebSocketService.Instance.search(form);
    } else {
      this.state.suggestedPosts = [];
    }

    this.setState(this.state);
  }

  handlePostBodyChange = (event: any) => {
    this.state.postForm.body = event.target.value;
    this.setState(this.state);
  }

  handlePostCommunityChange = (event: any) => {
    this.state.postForm.community_id = Number(event.target.value);
    this.setState(this.state);
  }

  handlePostNsfwChange = (event: any) => {
    this.state.postForm.nsfw = event.target.checked;
    this.setState(this.state);
  }

  handleCancel = () => {
    this.props.onCancel();
  }

  handlePreviewToggle = (event: any) => {
    event.preventDefault();
    this.state.previewMode = !this.state.previewMode;
    this.setState(this.state);
  }

  handleImageUploadPaste = (event: any) => {
    let image = event.clipboardData.files[0];
    if (image) {
      this.handleImageUpload(image);
    }
  }

  handleImageUpload = (event: any) => {
    let file: any;
    if (event.target) {
      event.preventDefault();
      file = event.target.files[0];
    } else {
      file = event;
    }

    const imageUploadUrl = `/pictshare/api/upload.php`;
    const formData = new FormData();
    formData.append('file', file);

    this.state.imageLoading = true;
    this.setState(this.state);

    fetch(imageUploadUrl, {
      method: 'POST',
      body: formData,
    })
      .then(res => res.json())
      .then(res => {
        let url = `${window.location.origin}/pictshare/${encodeURI(res.url)}`;
        if (res.filetype == 'mp4') {
          url += '/raw';
        }
        this.state.postForm.url = url;
        this.state.imageLoading = false;
        this.setState(this.state);
      })
      .catch(error => {
        this.state.imageLoading = false;
        this.setState(this.state);
        toast(error, 'danger');
      });
  }

  handleEmojiPickerClick = (event: any) => {
    emojiPicker.togglePicker(event.target);
  }

  parseMessage(msg: WebSocketJsonResponse) {
    let res = wsJsonToRes(msg);
    if (msg.error) {
      toast(i18n.t(msg.error), 'danger');
      this.state.loading = false;
      this.setState(this.state);
      return;
    } else if (res.op == UserOperation.ListCommunities) {
      let data = res.data as ListCommunitiesResponse;
      this.state.communities = data.communities;
      if (this.props.post) {
        this.state.postForm.community_id = this.props.post.community_id;
      } else if (this.props.params && this.props.params.community) {
        let foundCommunityId = data.communities.find(
          r => r.name == this.props.params.community
        ).id;
        this.state.postForm.community_id = foundCommunityId;
      } else {
        // By default, the null valued 'Select a Community'
      }
      this.setState(this.state);

      // Set up select searching
      let selectId: any = document.getElementById('post-community');
      if (selectId) {
        let selector = new Selectr(selectId, { nativeDropdown: false });
        selector.on('selectr.select', option => {
          this.state.postForm.community_id = Number(option.value);
          this.setState(this.state);
        });
      }
    } else if (res.op == UserOperation.CreatePost) {
      let data = res.data as PostResponse;
      if (data.post.creator_id == UserService.Instance.user.id) {
        this.state.loading = false;
        this.props.onCreate(data.post.id);
      }
    } else if (res.op == UserOperation.EditPost) {
      let data = res.data as PostResponse;
      if (data.post.creator_id == UserService.Instance.user.id) {
        this.state.loading = false;
        this.props.onEdit(data.post);
      }
    } else if (res.op == UserOperation.Search) {
      let data = res.data as SearchResponse;

      if (data.type_ == SearchType[SearchType.Posts]) {
        this.state.suggestedPosts = data.posts;
      } else if (data.type_ == SearchType[SearchType.Url]) {
        this.state.crossPosts = data.posts;
      }
      this.setState(this.state);
    } else if (res.op == UserOperation.GetSite) {
      let data = res.data as GetSiteResponse;
      this.state.enable_nsfw = data.site.enable_nsfw;
      this.setState(this.state);
    }
  }
}
