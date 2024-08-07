---
title: Export
swizzle: true
---

`<ExportButton>` is a Material UI [`<LoadingButton>`][button] with a default export icon and a default text with "Export". It only has presentational value.

> For more information, refer to the [useExport documentation &#8594][useexport]

:::simple Good to know

You can swizzle this component with the [**Refine CLI**](/docs/packages/list-of-packages) to customize it.

:::

## Usage

Use it like any other Ant Design [`<Button>`][button]. You can use it with [useExport][useexport]:

```tsx live url=http://localhost:3000/posts previewHeight=340px
// visible-block-start
import { useExport } from "@refinedev/core";
import {
  useDataGrid,
  List,
  // highlight-next-line
  ExportButton,
} from "@refinedev/mui";
import { DataGrid, GridColDef } from "@mui/x-data-grid";

const columns: GridColDef[] = [
  { field: "id", headerName: "ID", type: "number" },
  { field: "title", headerName: "Title", minWidth: 400, flex: 1 },
];

const PostsList: React.FC = () => {
  const { dataGridProps } = useDataGrid<IPost>();

  const { triggerExport, isLoading: exportLoading } = useExport<IPost>();

  return (
    <List
      // highlight-start
      headerButtons={<ExportButton onClick={triggerExport} loading={exportLoading} />}
      // highlight-end
    >
      <DataGrid {...dataGridProps} columns={columns} autoHeight />
    </List>
  );
};

interface IPost {
  id: number;
  title: string;
}
// visible-block-end

render(
  <RefineMuiDemo
    resources={[
      {
        name: "posts",
        list: PostsList,
      },
    ]}
  />,
);
```

## Properties

### hideText

`hideText` is used to show or hide text of the button. When `true`, only the button icon is visible.

```tsx live disableScroll previewHeight=120px
const { useRouterContext } = RefineCore;

// visible-block-start
import { ExportButton } from "@refinedev/mui";

const MyExportComponent = () => {
  return (
    <ExportButton
      // highlight-next-line
      hideText={true}
    />
  );
};

// visible-block-end

render(
  <RefineMuiDemo
    initialRoutes={["/"]}
    resources={[
      {
        name: "posts",
        list: MyExportComponent,
      },
    ]}
  />,
);
```

## API Reference

### Properties

<PropsTable module="@refinedev/mui/ExportButton" />

:::simple External Props

It also accepts all props of Material UI [LoadingButton](https://mui.com/material-ui/api/loading-button/#main-content).

:::

[button]: https://mui.com/material-ui/api/loading-button/#main-content
[useexport]: /docs/core/hooks/utilities/use-export
