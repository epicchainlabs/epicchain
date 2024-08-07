---
title: Email
swizzle: true
---

```tsx live shared
const { default: routerProvider } = LegacyRefineReactRouterV6;
const { default: simpleRest } = RefineSimpleRest;
setRefineProps({
  legacyRouterProvider: routerProvider,
  dataProvider: simpleRest("https://api.fake-rest."),
  Layout: RefineChakra.Layout,
  Sider: () => null,
});

const Wrapper = ({ children }) => {
  return <ChakraUI.ChakraProvider theme={RefineChakra.refineTheme}>{children}</ChakraUI.ChakraProvider>;
};
```

This field is used to display email values. It uses the [`<Link>`](https://chakra-ui.com/docs/components/link/usage) component of Chakra UI.

:::simple Good to know

You can swizzle this component to customize it with the [**Refine CLI**](/docs/packages/list-of-packages)

:::

## Usage

Let's see how we can use `<EmailField>` with the example in the user list.

```tsx live url=http://localhost:3000/users previewHeight=420px hideCode
setInitialRoutes(["/users"]);
import { Refine } from "@refinedev/core";

// visible-block-start
import {
  List,
  // highlight-next-line
  EmailField,
} from "@refinedev/chakra-ui";
import { TableContainer, Table, Thead, Tr, Th, Tbody, Td } from "@chakra-ui/react";
import { useTable } from "@refinedev/react-table";
import { ColumnDef, flexRender } from "@tanstack/react-table";

const UserList: React.FC = () => {
  const columns = React.useMemo<ColumnDef<IUser>[]>(
    () => [
      {
        id: "id",
        header: "ID",
        accessorKey: "id",
      },
      {
        id: "firstName",
        header: "First Name",
        accessorKey: "firstName",
      },
      {
        id: "lastName",
        header: "Last Name",
        accessorKey: "lastName",
      },
      {
        id: "email",
        header: "Email",
        accessorKey: "email",
        cell: function render({ getValue }) {
          return (
            // highlight-next-line
            <EmailField value={getValue()} />
          );
        },
      },
    ],
    [],
  );

  const { getHeaderGroups, getRowModel } = useTable({
    columns,
  });

  return (
    <List>
      <TableContainer>
        <Table variant="simple" whiteSpace="pre-line">
          <Thead>
            {getHeaderGroups().map((headerGroup) => (
              <Tr key={headerGroup.id}>
                {headerGroup.headers.map((header) => {
                  return (
                    <Th key={header.id}>
                      {!header.isPlaceholder && flexRender(header.column.columnDef.header, header.getContext())}
                    </Th>
                  );
                })}
              </Tr>
            ))}
          </Thead>
          <Tbody>
            {getRowModel().rows.map((row) => {
              return (
                <Tr key={row.id}>
                  {row.getVisibleCells().map((cell) => {
                    return <Td key={cell.id}>{flexRender(cell.column.columnDef.cell, cell.getContext())}</Td>;
                  })}
                </Tr>
              );
            })}
          </Tbody>
        </Table>
      </TableContainer>
    </List>
  );
};

interface IUser {
  id: number;
  firstName: string;
  lastName: string;
  email: string;
}
// visible-block-end

const App = () => {
  return (
    <Refine
      notificationProvider={RefineChakra.notificationProvider()}
      resources={[{ name: "users", list: UserList }]}
    />
  );
};

render(
  <Wrapper>
    <App />
  </Wrapper>,
);
```

:::tip

`<EmailField>` uses "mailto:" in the href prop of the [`<Link>`](https://chakra-ui.com/docs/components/link/usage) component. For this reason, clicking `<EmailField>` opens your device's default mail application.

:::

## API Reference

### Properties

<PropsTable module="@refinedev/chakra-ui/EmailField" />

> For the rest of Anchor properties, refer to the [Chakra UI documentation &#8594](https://chakra-ui.com/docs/components/link/usage)
