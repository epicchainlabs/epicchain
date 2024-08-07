---
title: Clone
swizzle: true
---

```tsx live shared
const { default: routerProvider } = LegacyRefineReactRouterV6;
const { default: simpleRest } = RefineSimpleRest;
setRefineProps({
  legacyRouterProvider: routerProvider,
  dataProvider: simpleRest("https://api.fake-rest."),
  notificationProvider: RefineMantine.useNotificationProvider,
  Layout: RefineMantine.Layout,
  Sider: () => null,
  catchAll: <RefineMantine.ErrorComponent />,
});

const Wrapper = ({ children }) => {
  return (
    <MantineCore.MantineProvider theme={RefineMantine.LightTheme} withNormalizeCSS withGlobalStyles>
      <MantineCore.Global styles={{ body: { WebkitFontSmoothing: "auto" } }} />
      <MantineNotifications.NotificationsProvider position="top-right">
        {children}
      </MantineNotifications.NotificationsProvider>
    </MantineCore.MantineProvider>
  );
};

const ClonePage = () => {
  const { list } = RefineCore.useNavigation();
  const params = RefineCore.useRouterContext().useParams();

  return (
    <div>
      <MantineCore.Text italic color="dimmed" size="sm">
        URL Parameters:
      </MantineCore.Text>
      <MantineCore.Code>{JSON.stringify(params)}</MantineCore.Code>
      <MantineCore.Space h="md" />
      <MantineCore.Button size="xs" variant="outline" onClick={() => list("posts")}>
        Go back
      </MantineCore.Button>
    </div>
  );
};
```

`<CloneButton>` uses Mantine's [`<Button>`](https://mantine.dev/core/button) component. It uses the `clone` method from [useNavigation](/docs/routing/hooks/use-navigation) under the hood.
It can be useful when redirecting the app to the create page with the record id route of resource.

:::simple Good to know

You can swizzle this component to customize it with the [**Refine CLI**](/docs/packages/list-of-packages)

:::

## Usage

```tsx live url=http://localhost:3000 previewHeight=420px hideCode
setInitialRoutes(["/posts"]);
import { Refine } from "@refinedev/core";

// visible-block-start
import { List, CloneButton } from "@refinedev/mantine";
import { Table, Pagination } from "@mantine/core";

import { useTable } from "@refinedev/react-table";
import { ColumnDef, flexRender } from "@tanstack/react-table";

const PostList: React.FC = () => {
  const columns = React.useMemo<ColumnDef<IPost>[]>(
    () => [
      {
        id: "id",
        header: "ID",
        accessorKey: "id",
      },
      {
        id: "title",
        header: "Title",
        accessorKey: "title",
      },
      {
        id: "actions",
        header: "Actions",
        accessorKey: "id",
        cell: function render({ getValue }) {
          return (
            // highlight-start
            <CloneButton size="xs" recordItemId={getValue() as number} />
            // highlight-end
          );
        },
      },
    ],
    [],
  );

  const {
    getHeaderGroups,
    getRowModel,
    refineCore: { setCurrent, pageCount, current },
  } = useTable({
    columns,
  });

  //hide-start
  List.defaultProps = {
    headerButtons: <></>,
  };
  //hide-end

  return (
    <List>
      <Table>
        <thead>
          {getHeaderGroups().map((headerGroup) => (
            <tr key={headerGroup.id}>
              {headerGroup.headers.map((header) => (
                <th key={header.id}>
                  {header.isPlaceholder ? null : flexRender(header.column.columnDef.header, header.getContext())}
                </th>
              ))}
            </tr>
          ))}
        </thead>
        <tbody>
          {getRowModel().rows.map((row) => (
            <tr key={row.id}>
              {row.getVisibleCells().map((cell) => (
                <td key={cell.id}>{flexRender(cell.column.columnDef.cell, cell.getContext())}</td>
              ))}
            </tr>
          ))}
        </tbody>
      </Table>
      <br />
      <Pagination position="right" total={pageCount} page={current} onChange={setCurrent} />
    </List>
  );
};

interface IPost {
  id: number;
  title: string;
}
// visible-block-end

const App = () => {
  return (
    <RefineHeadlessDemo
      resources={[
        {
          name: "posts",
          list: PostList,
          create: ClonePage,
        },
      ]}
    />
  );
};
render(
  <Wrapper>
    <App />
  </Wrapper>,
);
```

## Properties

### recordItemId

`recordItemId` is used to append the record id to the end of the route path. By default `id` will be read from the route parameters.

```tsx live url=http://localhost:3000 previewHeight=200px
setInitialRoutes(["/"]);
import { Refine } from "@refinedev/core";

// visible-block-start
import { CloneButton } from "@refinedev/mantine";

const MyCloneComponent = () => {
  return <CloneButton recordItemId="123" />;
};
// visible-block-end

const App = () => {
  return (
    <RefineHeadlessDemo
      resources={[
        {
          name: "posts",
          create: ClonePage,
          list: MyCloneComponent,
        },
      ]}
    />
  );
};

render(
  <Wrapper>
    <App />
  </Wrapper>,
);
```

Clicking the button will trigger the `clone` method of [`useNavigation`](/docs/routing/hooks/use-navigation) and then redirect the app to the `clone` action path of the resource, filling the necessary parameters in the route.

### resource

It is used to redirect the app to the `clone` action of the given resource name. By default, the app redirects to the inferred resource's `clone` action path.

```tsx live url=http://localhost:3000 previewHeight=200px
setInitialRoutes(["/"]);

import { Refine } from "@refinedev/core";

// visible-block-start
import { CloneButton } from "@refinedev/mantine";

const MyCloneComponent = () => {
  return <CloneButton resource="categories" recordItemId="2" />;
};
// visible-block-end

const App = () => {
  return (
    <RefineHeadlessDemo
      resources={[
        {
          name: "posts",
          list: MyCloneComponent,
        },
        {
          name: "categories",
          create: ClonePage,
        },
      ]}
    />
  );
};

render(
  <Wrapper>
    <App />
  </Wrapper>,
);
```

Clicking the button will trigger the `clone` method of [`useNavigation`](/docs/routing/hooks/use-navigation) and then redirect the app to the `clone` action path of the resource, filling the necessary parameters in the route.

### meta

It is used to pass additional parameters to the `clone` method of [`useNavigation`](/docs/routing/hooks/use-navigation). By default, existing parameters in the route are used by the `clone` method. You can pass additional parameters or override the existing ones using the `meta` prop.

If the `clone` action route is defined by the pattern: `/posts/:authorId/clone/:id`, the `meta` prop can be used as follows:

```tsx
const MyComponent = () => {
  return <CloneButton meta={{ authorId: "10" }} />;
};
```

If you have multiple resources with the same name, you can pass the `identifier` instead of the `name` of the resource. It will only be used as the main matching key for the resource, data provider methods will still work with the `name` of the resource defined in the `<Refine/>` component.

> For more information, refer to the [`identifier` section of the `<Refine/>` component documentation &#8594](/docs/core/refine-component#identifier)

### hideText

`hideText` is used to show and not show the text of the button. When `true`, only the button icon is visible.

```tsx live url=http://localhost:3000 previewHeight=200px
setInitialRoutes(["/"]);

import { Refine } from "@refinedev/core";

// visible-block-start
import { CloneButton } from "@refinedev/mantine";

const MyCloneComponent = () => {
  return <CloneButton hideText />;
};
// visible-block-end

const App = () => {
  return (
    <RefineHeadlessDemo
      resources={[
        {
          name: "posts",
          list: MyCloneComponent,
          create: ClonePage,
        },
      ]}
    />
  );
};

render(
  <Wrapper>
    <App />
  </Wrapper>,
);
```

### accessControl

This prop can be used to skip access control check with its `enabled` property or to hide the button when the user does not have the permission to access the resource with `hideIfUnauthorized` property. This is relevant only when an [`accessControlProvider`](/docs/authorization/access-control-provider) is provided to [`<Refine/>`](/docs/core/refine-component)

```tsx
import { CloneButton } from "@refinedev/mantine";

export const MyListComponent = () => {
  return <CloneButton accessControl={{ enabled: true, hideIfUnauthorized: true }} />;
};
```

### ~~resourceNameOrRouteName~~ <PropTag deprecated />

Use `resource` prop instead.

## API Reference

### Properties

<PropsTable module="@refinedev/mantine/CloneButton" />
