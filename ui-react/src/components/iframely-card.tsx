import React, { Component } from 'react';
import { Post } from '../interfaces';
import { mdToHtml } from '../utils';
import { i18n } from '../i18next';

interface FramelyCardProps {
  post: Post;
}

interface FramelyCardState {
  expanded: boolean;
}

export class IFramelyCard extends Component<
  FramelyCardProps,
  FramelyCardState
> {

  state : FramelyCardState;

  private emptyState: FramelyCardState = {
    expanded: false,
  };

  constructor(props: any, context: any) {
    super(props, context);
    this.state = this.emptyState;
  }

  render() {
    let post = this.props.post;
    return (
      <>
        {post.embed_title && !this.state.expanded && (
          <div className="card mt-3 mb-2">
            <div className="row">
              <div className="col-12">
                <div className="card-body">
                  <h5 className="card-title d-inline">
                    {post.embed_html ? (
                      <span
                        className="unselectable pointer"
                        onClick={this.handleIframeExpand}
                        data-tippy-content={i18n.t('expand_here')}
                      >
                        {post.embed_title}
                      </span>
                    ) : (
                      <span>
                        <a className="text-body" target="_blank" href={post.url}>
                          {post.embed_title}
                        </a>
                      </span>
                    )}
                  </h5>
                  <span className="d-inline-block ml-2 mb-2 small text-muted">
                    <a
                      className="text-muted font-italic"
                      target="_blank"
                      href={post.url}
                    >
                      {new URL(post.url).hostname}
                      <svg className="ml-1 icon">
                        <use xlinkHref="#icon-external-link"></use>
                      </svg>
                    </a>
                    {post.embed_html && (
                      <span
                        className="ml-2 pointer text-monospace"
                        onClick={this.handleIframeExpand}
                        data-tippy-content={i18n.t('expand_here')}
                      >
                        {this.state.expanded ? '[-]' : '[+]'}
                      </span>
                    )}
                  </span>
                  {post.embed_description && (
                    <div
                      className="card-text small text-muted md-div"
                      dangerouslySetInnerHTML={mdToHtml(post.embed_description)}
                    />
                  )}
                </div>
              </div>
            </div>
          </div>
        )}
        {this.state.expanded && (
          <div
            className="mt-3 mb-2"
            dangerouslySetInnerHTML={{ __html: post.embed_html }}
          />
        )}
      </>
    );
  }

  handleIframeExpand = () => {
    this.state.expanded = !this.state.expanded;
    this.setState(this.state);
  }
}
