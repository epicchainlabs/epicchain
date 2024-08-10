---
title: Show
swizzle: true
---

`<Show>` provides us a layout for displaying the page. It does not contain any logic but adds extra functionalities like a refresh button or giving title to the page.

We will show what `<Show>` does using properties with examples.

```tsx live hideCode url=http://localhost:3000/posts/show/2
// visible-block-start
import { Show, MarkdownField } from "@refinedev/antd";
import { Typography } from "antd";
import { useShow, useOne } from "@refinedev/core";

const { Title, Text } = Typography;

const PostShow: React.FC = () => {
  const { queryResult } = useShow<IPost>();
  const { data, isLoading } = queryResult;
  const record = data?.data;

  const { data: categoryData, isLoading: categoryIsLoading } = useOne<ICategory>({
    resource: "categories",
    id: record?.category.id || "",
    queryOptions: {
      enabled: !!record,
    },
  });

  return (
    <Show isLoading={isLoading}>
      <Title level={5}>Id</Title>
      <Text>{record?.id}</Text>

      <Title level={5}>Title</Title>
      <Text>{record?.title}</Text>

      <Title level={5}>Category</Title>
      <Text>{categoryIsLoading ? "Loading..." : categoryData?.data.title}</Text>

      <Title level={5}>Content</Title>
      <MarkdownField value={record?.content} />
    </Show>
  );
};
// visible-block-end

render(
  <RefineAntdDemo
    initialRoutes={["/posts/show/2"]}
    resources={[
      {
        name: "posts",
        list: () => (
          <div>
            <p>This page is empty.</p>
            <ShowButton recordItemId="2">Edit Item 2</ShowButton>
          </div>
        ),
        show: PostShow,
      },
    ]}
  />,
);
```

:::simple Good to know

You can swizzle this component to customize it with the [**Refine CLI**](/docs/packages/list-of-packages)

:::

## Properties

### title

`title` allows you to add a title inside the `<Show>` component. If you don't pass title props, it uses the "Show" prefix and the singular resource name by default. For example, for the "posts" resource, it will be "Show post".

```tsx live disableScroll previewHeight=280px url=http://localhost:3000/posts/show/2
const { ShowButton } = RefineAntd;
const { default: simpleRest } = RefineSimpleRest;

const dataProvider = simpleRest("https://api.fake-rest.");

const customDataProvider = {
  ...dataProvider,
  deleteOne: async ({ resource, id, variables }) => {
    return {
      data: {},
    };
  },
};

// visible-block-start
import { Show } from "@refinedev/antd";

const PostShow: React.FC = () => {
  return (
    /* highlight-next-line */
    <Show title="Custom Title">
      <p>Rest of your page here</p>
    </Show>
  );
};
// visible-block-end

render(
  <RefineAntdDemo
    initialRoutes={["/posts/show/2"]}
    resources={[
      {
        name: "posts",
        list: () => (
          <div>
            <p>This page is empty.</p>
            <ShowButton recordItemId="2">Show Item 2</ShowButton>
          </div>
        ),
        show: PostShow,
      },
    ]}
  />,
);
```

### resource

The `<Show>` component reads the `resource` information from the route by default. If you want to use a custom resource for the `<Show>` component, you can use the `resource` prop:

```tsx live disableScroll previewHeight=280px url=http://localhost:3000/custom/2
setInitialRoutes(["/custom/2"]);

import { Refine } from "@refinedev/core";
import routerProvider from "@refinedev/react-router-v6/legacy";
import dataProvider from "@refinedev/simple-rest";
// visible-block-start
import { Show } from "@refinedev/antd";

const CustomPage: React.FC = () => {
  return (
    /* highlight-next-line */
    <Show resource="posts">
      <p>Rest of your page here</p>
    </Show>
  );
};
// visible-block-end
const App: React.FC = () => {
  return (
    <RefineAntdDemo
      legacyRouterProvider={{
        ...routerProvider,
        // highlight-start
        routes: [
          {
            element: <CustomPage />,
            path: "/custom/:id",
          },
        ],
        // highlight-end
      }}
      dataProvider={dataProvider("https://api.fake-rest.")}
      resources={[{ name: "posts" }]}
    />
  );
};

render(<App />);
```

If you have multiple resources with the same name, you can pass the `identifier` instead of the `name` of the resource. It will only be used as the main matching key for the resource, data provider methods will still work with the `name` of the resource defined in the `<Refine/>` component.

> For more information, refer to the [`identifier` of the `<Refine/>` component documentation &#8594](/docs/core/refine-component#identifier)

### canDelete and canEdit

`canDelete` and `canEdit` allows us to add the delete and edit buttons inside the `<Show>` component.

When clicked on, the delete button executes the `useDelete` method provided by the [`dataProvider`](/docs/data/data-provider) and the edit button redirects the user to the record edit page.

```tsx live disableScroll previewHeight=280px url=http://localhost:3000/posts/show/2
const { ShowButton, Edit } = RefineAntd;

const { default: simpleRest } = RefineSimpleRest;

const dataProvider = simpleRest("https://api.fake-rest.");

const customDataProvider = {
  ...dataProvider,
  deleteOne: async ({ resource, id, variables }) => {
    return {
      data: {},
    };
  },
};

const authProvider = {
  login: async () => {
    return {
      success: true,
      redirectTo: "/",
    };
  },
  register: async () => {
    return {
      success: true,
    };
  },
  forgotPassword: async () => {
    return {
      success: true,
    };
  },
  updatePassword: async () => {
    return {
      success: true,
    };
  },
  logout: async () => {
    return {
      success: true,
      redirectTo: "/",
    };
  },
  check: () => ({
    authenticated: true,
  }),
  onError: async (error) => {
    console.error(error);
    return { error };
  },
  getPermissions: async () => ["admin"],
  getIdentity: async () => null,
};

// visible-block-start
import { Show } from "@refinedev/antd";
import { usePermissions } from "@refinedev/core";

const PostShow: React.FC = () => {
  const { data: permissionsData } = usePermissions();
  return (
    <Show
      /* highlight-start */
      canDelete={permissionsData?.includes("admin")}
      canEdit={permissionsData?.includes("admin")}
      /* highlight-end */
    >
      <p>Rest of your page here</p>
    </Show>
  );
};
// visible-block-end

render(
  <RefineAntdDemo
    authProvider={authProvider}
    dataProvider={customDataProvider}
    initialRoutes={["/posts/show/2"]}
    resources={[
      {
        name: "posts",
        list: () => (
          <div>
            <p>This page is empty.</p>
            <ShowButton>Show Item 2</ShowButton>
          </div>
        ),
        show: PostShow,
        edit: () => {
          return <Edit>Edit Page</Edit>;
        },
      },
    ]}
  />,
);
```

> For more information, refer to the [`usePermission` documentation &#8594](/docs/authentication/hooks/use-permissions)

> For more information, refer to the [`<DeleteButton>`](/docs/ui-integrations/ant-design/components/buttons/delete-button) and the [`<EditButton>`](/docs/ui-integrations/ant-design/components/buttons/edit-button) documentations.

### recordItemId

The`<Show>` component reads the `id` information from the route by default. `recordItemId` is used when it cannot read from the URL (when used on a custom page, modal or drawer).

```tsx live disableScroll previewHeight=280px url=http://localhost:3000/posts/show/2
const { ShowButton } = RefineAntd;

// visible-block-start
import { Show, useModalForm } from "@refinedev/antd";
import { Modal, Button } from "antd";

const PostShow: React.FC = () => {
  const { modalProps, id, show } = useModalForm({
    action: "show",
  });

  return (
    <div>
      <Button onClick={() => show()}>Show Button</Button>
      <Modal {...modalProps}>
        {/* highlight-next-line */}
        <Show recordItemId={id}>
          <p>Rest of your page here</p>
        </Show>
      </Modal>
    </div>
  );
};
// visible-block-end

render(
  <RefineAntdDemo
    initialRoutes={["/posts/show/2"]}
    resources={[
      {
        name: "posts",
        list: () => (
          <div>
            <p>This page is empty.</p>
            <ShowButton recordItemId="2">Show Item 2</ShowButton>
          </div>
        ),
        show: PostShow,
      },
    ]}
  />,
);
```

The `<Show>` component needs the `id` information for `<RefreshButton>` to work properly.

### dataProviderName

If not specified, Refine will use the default data provider. If you have multiple data providers, you can use the `dataProviderName` property to specify which one you want to use:

```tsx
import { Refine } from "@refinedev/core";
import dataProvider from "@refinedev/simple-rest";

import { Show } from "@refinedev/antd";

// highlight-start
const PostShow = () => {
  return <Show dataProviderName="other">...</Show>;
};
// highlight-end

export const App: React.FC = () => {
  return (
    <Refine
      // highlight-start
      dataProvider={{
        default: dataProvider("https://api.fake-rest./"),
        other: dataProvider("https://other-api.fake-rest./"),
      }}
      // highlight-end
    >
      {/* ... */}
    </Refine>
  );
};
```

### goBack

To customize the back button or to disable it, you can use the `goBack` property:

```tsx live disableScroll previewHeight=280px url=http://localhost:3000/posts/show/2
const { ShowButton } = RefineAntd;

// visible-block-start
import { Show } from "@refinedev/antd";
import { Button } from "antd";

const PostShow: React.FC = () => {
  const BackButton = () => <Button>←</Button>;
  return (
    /* highlight-next-line */
    <Show goBack={<div>back</div>}>
      <p>Rest of your page here</p>
    </Show>
  );
};
// visible-block-end

render(
  <RefineAntdDemo
    initialRoutes={["/posts", "/posts/show/2"]}
    resources={[
      {
        name: "posts",
        list: () => (
          <div>
            <p>This page is empty.</p>
            <ShowButton>Show Item 2</ShowButton>
          </div>
        ),
        show: PostShow,
      },
    ]}
  />,
);
```

If your route has no `:action` parameter or your action is `list`, the back button will not be shown even if you pass a `goBack` property. You can override this behavior by using the `headerProps` property:

```tsx
/* highlight-next-line */
import { useBack } from "@refinedev/core";
import { Show } from "@refinedev/antd";
import { Button } from "antd";

const PostShow: React.FC = () => {
  /* highlight-next-line */
  const back = useBack();
  const BackButton = () => <Button>←</Button>;

  return (
    /* highlight-next-line */
    <Show goBack={<BackButton />} headerProps={{ onBack: back }}>
      <p>Rest of your page here</p>
    </Show>
  );
};
```

### isLoading

Since `<Show>` uses the Ant Design [`<Card>`](https://ant.design/components/card/) component, the `isLoading` property can be set like the below:

```tsx live disableScroll previewHeight=280px url=http://localhost:3000/posts/show/2
const { ShowButton } = RefineAntd;

// visible-block-start
import { Show } from "@refinedev/antd";

const PostShow: React.FC = () => {
  return (
    /* highlight-next-line */
    <Show isLoading={true}>
      <p>Rest of your page here</p>
    </Show>
  );
};
// visible-block-end

render(
  <RefineAntdDemo
    initialRoutes={["/posts/show/2"]}
    resources={[
      {
        name: "posts",
        list: () => (
          <div>
            <p>This page is empty.</p>
            <ShowButton />
          </div>
        ),
        show: PostShow,
      },
    ]}
  />,
);
```

### breadcrumb <GlobalConfigBadge id="core/refine-component/#breadcrumb" />

To customize or disable the breadcrumb, you can use the `breadcrumb` property. By default the `Breadcrumb` component from the `@refinedev/antd` package is used for breadcrumbs.

```tsx live disableScroll previewHeight=280px url=http://localhost:3000/posts/show/2
const { ShowButton } = RefineAntd;

// visible-block-start
import { Show, Breadcrumb } from "@refinedev/antd";

const PostShow: React.FC = () => {
  return (
    <Show
      // highlight-start
      breadcrumb={
        <div
          style={{
            padding: "3px 6px",
            border: "2px dashed cornflowerblue",
          }}
        >
          <Breadcrumb />
        </div>
      }
      // highlight-end
    >
      <p>Rest of your page here</p>
    </Show>
  );
};
// visible-block-end

render(
  <RefineAntdDemo
    initialRoutes={["/posts/show"]}
    resources={[
      {
        name: "posts",
        list: () => (
          <div>
            <p>This page is empty.</p>
            <ShowButton />
          </div>
        ),
        show: PostShow,
      },
    ]}
  />,
);
```

> For more information, refer to the [`Breadcrumb` documentation &#8594](/docs/ui-integrations/ant-design/components/breadcrumb)

### wrapperProps

You can use the `wrapperProps` property if you want to customize the wrapper of the `<Show/>` component. The `@refinedev/antd` wrapper elements are simply `<div/>`s and `wrapperProps` and can get every attribute that `<div/>` can get.

```tsx live disableScroll previewHeight=280px url=http://localhost:3000/posts/show/2
const { ShowButton } = RefineAntd;

// visible-block-start
import { Show } from "@refinedev/antd";

const PostShow: React.FC = () => {
  return (
    <Show
      // highlight-start
      wrapperProps={{
        style: {
          backgroundColor: "cornflowerblue",
          padding: "16px",
        },
      }}
      // highlight-end
    >
      <p>Rest of your page here</p>
    </Show>
  );
};
// visible-block-end

render(
  <RefineAntdDemo
    initialRoutes={["/posts/show"]}
    resources={[
      {
        name: "posts",
        list: () => (
          <div>
            <p>This page is empty.</p>
            <ShowButton />
          </div>
        ),
        show: PostShow,
      },
    ]}
  />,
);
```

### headerProps

You can use the `headerProps` property to customize the header of the `<Show/>` component:

```tsx live disableScroll previewHeight=280px url=http://localhost:3000/posts/show/2
const { ShowButton } = RefineAntd;

// visible-block-start
import { Show } from "@refinedev/antd";

const PostShow: React.FC = () => {
  return (
    <Show
      // highlight-start
      headerProps={{
        subTitle: "This is a subtitle",
        style: {
          backgroundColor: "cornflowerblue",
          padding: "16px",
        },
      }}
      // highlight-end
    >
      <p>Rest of your page here</p>
    </Show>
  );
};
// visible-block-end

render(
  <RefineAntdDemo
    initialRoutes={["/posts/show/2"]}
    resources={[
      {
        name: "posts",
        list: () => (
          <div>
            <p>This page is empty.</p>
            <ShowButton />
          </div>
        ),
        show: PostShow,
      },
    ]}
  />,
);
```

> For more information, refer to the [`PageHeader` documentation &#8594](https://procomponents.ant.design/en-US/components/page-header)

### contentProps

You can use the `contentProps` property to customize the content of the `<Show/>` component:

```tsx live disableScroll previewHeight=280px url=http://localhost:3000/posts/show/2
const { ShowButton } = RefineAntd;

// visible-block-start
import { Show } from "@refinedev/antd";

const PostShow: React.FC = () => {
  return (
    <Show
      // highlight-start
      contentProps={{
        style: {
          backgroundColor: "cornflowerblue",
          padding: "16px",
        },
      }}
      // highlight-end
    >
      <p>Rest of your page here</p>
    </Show>
  );
};
// visible-block-end

render(
  <RefineAntdDemo
    initialRoutes={["/posts/show"]}
    resources={[
      {
        name: "posts",
        list: () => (
          <div>
            <p>This page is empty.</p>
            <ShowButton />
          </div>
        ),
        show: PostShow,
      },
    ]}
  />,
);
```

> For more information, refer to the [`Card` documentation &#8594](https://ant.design/components/card/)

### headerButtons

By default, the `<Show/>` component has a [`<ListButton>`][list-button], [`<EditButton>`][edit-button], [`<DeleteButton>`][delete-button], and a [`<RefreshButton>`][refresh-button] at the header.

You can customize the buttons at the header by using the `headerButtons` property. It accepts `React.ReactNode` or a render function `({ defaultButtons, listButtonProps, editButtonProps, deleteButtonProps, refreshButtonProps }) => React.ReactNode` which you can use to keep the existing buttons and add your own.

If the "list" resource is not defined, the [`<ListButton>`][list-button] will not render and `listButtonProps` will be `undefined`.

If [`canDelete`](#candelete-and-canedit) is `false`, the [`<DeleteButton>`][delete-button] will not render and `deleteButtonProps` will be `undefined`.

If [`canEdit`](#candelete-and-canedit) is `false`, [`<EditButton>`][edit-button] will not render and `editButtonProps` will be `undefined`.

```tsx live disableScroll previewHeight=280px url=http://localhost:3000/posts/show/2
const { ShowButton } = RefineAntd;

// visible-block-start
import { Show } from "@refinedev/antd";
import { Button } from "antd";

const PostShow: React.FC = () => {
  return (
    <Show
      // highlight-start
      headerButtons={({ defaultButtons }) => (
        <>
          {defaultButtons}
          <Button type="primary">Custom Button</Button>
        </>
      )}
      // highlight-end
    >
      <p>Rest of your page here</p>
    </Show>
  );
};
// visible-block-end

render(
  <RefineAntdDemo
    initialRoutes={["/posts/show/2"]}
    resources={[
      {
        name: "posts",
        list: () => (
          <div>
            <p>This page is empty.</p>
            <ShowButton />
          </div>
        ),
        show: PostShow,
      },
    ]}
  />,
);
```

Or, instead of using the `defaultButtons`, you can create your own buttons. If you want, you can use `createButtonProps` to utilize the default values of the [`<ListButton>`][list-button], [`<EditButton>`][edit-button], [`<DeleteButton>`][delete-button], and [`<RefreshButton>`][refresh-button] components.

```tsx live disableScroll previewHeight=280px url=http://localhost:3000/posts/show/2
const { ShowButton } = RefineAntd;

// visible-block-start
import { Show, ListButton, EditButton, DeleteButton, RefreshButton } from "@refinedev/antd";
import { Button } from "antd";

const PostShow: React.FC = () => {
  return (
    <Show
      // highlight-start
      headerButtons={({ deleteButtonProps, editButtonProps, listButtonProps, refreshButtonProps }) => (
        <>
          <Button type="primary">Custom Button</Button>
          {listButtonProps && <ListButton {...listButtonProps} meta={{ foo: "bar" }} />}
          {editButtonProps && <EditButton {...editButtonProps} meta={{ foo: "bar" }} />}
          {deleteButtonProps && <DeleteButton {...deleteButtonProps} meta={{ foo: "bar" }} />}
          <RefreshButton {...refreshButtonProps} meta={{ foo: "bar" }} />
        </>
      )}
      // highlight-end
    >
      <p>Rest of your page here</p>
    </Show>
  );
};
// visible-block-end

render(
  <RefineAntdDemo
    initialRoutes={["/posts/show/2"]}
    resources={[
      {
        name: "posts",
        list: () => (
          <div>
            <p>This page is empty.</p>
            <ShowButton />
          </div>
        ),
        show: PostShow,
      },
    ]}
  />,
);
```

### headerButtonProps

You can use the `headerButtonProps` property to customize the wrapper element of the buttons at the header:

```tsx live disableScroll previewHeight=280px url=http://localhost:3000/posts/edit/2
const { ShowButton } = RefineAntd;

// visible-block-start
import { Show } from "@refinedev/antd";
import { Button } from "antd";

const PostShow: React.FC = () => {
  return (
    <Show
      // highlight-start
      headerButtonProps={{
        style: {
          backgroundColor: "cornflowerblue",
          padding: "16px",
        },
      }}
      // highlight-end
      headerButtons={<Button type="primary">Custom Button</Button>}
    >
      <p>Rest of your page here</p>
    </Show>
  );
};
// visible-block-end

render(
  <RefineAntdDemo
    initialRoutes={["/posts/show"]}
    resources={[
      {
        name: "posts",
        list: () => (
          <div>
            <p>This page is empty.</p>
            <ShowButton />
          </div>
        ),
        show: PostShow,
      },
    ]}
  />,
);
```

> For more information, refer to the [`Space` documentation &#8594](https://ant.design/components/space/)

### footerButtons

You can customize the buttons at the footer by using the `footerButtons` property. It accepts `React.ReactNode` or a render function `({ defaultButtons }) => React.ReactNode` which you can use to keep the existing buttons and add your own.

```tsx live disableScroll previewHeight=280px url=http://localhost:3000/posts/show/2
const { ShowButton } = RefineAntd;

// visible-block-start
import { Show } from "@refinedev/antd";
import { Button } from "antd";

const PostShow: React.FC = () => {
  return (
    <Show
      // highlight-start
      footerButtons={({ defaultButtons }) => (
        <>
          {defaultButtons}
          <Button type="primary">Custom Button</Button>
        </>
      )}
      // highlight-end
    >
      <p>Rest of your page here</p>
    </Show>
  );
};
// visible-block-end

render(
  <RefineAntdDemo
    initialRoutes={["/posts/show"]}
    resources={[
      {
        name: "posts",
        list: () => (
          <div>
            <p>This page is empty.</p>
            <ShowButton />
          </div>
        ),
        show: PostShow,
      },
    ]}
  />,
);
```

### footerButtonProps

You can customize the wrapper element of the buttons at the footer by using the `footerButtonProps` property.

```tsx live disableScroll previewHeight=280px url=http://localhost:3000/posts/show/2
const { ShowButton } = RefineAntd;

// visible-block-start
import { Show } from "@refinedev/antd";
import { Button } from "antd";

const PostShow: React.FC = () => {
  return (
    <Show
      footerButtons={({ defaultButtons }) => (
        <>
          {defaultButtons}
          <Button type="primary">Custom Button</Button>
        </>
      )}
      // highlight-start
      footerButtonProps={{
        style: {
          // hide-start
          float: "right",
          marginRight: 24,
          // hide-end
          backgroundColor: "cornflowerblue",
          padding: "16px",
        },
      }}
      // highlight-end
    >
      <p>Rest of your page here</p>
    </Show>
  );
};
// visible-block-end

render(
  <RefineAntdDemo
    initialRoutes={["/posts/show"]}
    resources={[
      {
        name: "posts",
        list: () => (
          <div>
            <p>This page is empty.</p>
            <ShowButton />
          </div>
        ),
        show: PostShow,
      },
    ]}
  />,
);
```

> For more information, refer to the [`Space` documentation &#8594](https://ant.design/components/space/)

## API Reference

### Properties

<PropsTable module="@refinedev/antd/Show"
contentProps-type="[`CardProps`](https://ant.design/components/card/#API)"
headerProps-type="[`PageHeaderProps`](https://procomponents.ant.design/en-US/components/page-header)"
headerButtons-default="[`ListButton`](/docs/ui-integrations/ant-design/components/buttons/list-button/), [`RefreshButton`](/docs/ui-integrations/ant-design/components/buttons/refresh-button/), [`EditButton`](/docs/ui-integrations/ant-design/components/buttons/edit-button/) and [`DeleteButton`](/docs/ui-integrations/ant-design/components/buttons/delete-button/)"
headerButtonProps-type="[`SpaceProps`](https://ant.design/components/space/)"
deleteButtonProps-type="[`DeleteButtonProps`](/docs/ui-integrations/ant-design/components/buttons/delete-button)"
saveButtonProps-type="[`SaveButtonProps`](/docs/ui-integrations/ant-design/components/buttons/save-button/)"
footerButtonsProps-type="[`SpaceProps`](https://ant.design/components/space/)"
breadcrumb-default="[`<Breadcrumb>`](https://ant.design/components/breadcrumb/)"
goBack-default="`<ArrowLeft />`"
goBack-type="`ReactNode`"
/>

[list-button]: /docs/ui-integrations/ant-design/components/buttons/list-button
[refresh-button]: /docs/ui-integrations/ant-design/components/buttons/refresh-button
[edit-button]: /docs/ui-integrations/ant-design/components/buttons/edit-button
[delete-button]: /docs/ui-integrations/ant-design/components/buttons/delete-button
