
import { ColumnDef } from "@tanstack/react-table"
import { Checkbox } from "../ui/checkbox"
import { Link } from "react-router-dom"

// This type is used to define the shape of our data.
// You can use a Zod schema here if you want.
export type PasteRow = {
  id: string
  expire: string
  burn: string
  views: number
}

export const columns: ColumnDef<PasteRow>[] = [
  {
    id: "select",
    header: ({ table }) => (
      <Checkbox
        checked={
          table.getIsAllPageRowsSelected() ||
          (table.getIsSomePageRowsSelected() && "indeterminate")
        }
        onCheckedChange={(value) => table.toggleAllPageRowsSelected(!!value)}
        aria-label="Select all"
      />
    ),
    cell: ({ row }) => (
      <Checkbox
        checked={row.getIsSelected()}
        onCheckedChange={(value) => row.toggleSelected(!!value)}
        aria-label="Select row"
      />
    ),
    enableSorting: false,
    enableHiding: false,
  },
  {
    accessorKey: "id",
    header: "Id",
    cell: ({ row }) => (
      <Link to={"/"+row.getValue("id")}>{row.getValue("id")}</Link>
    ),
  },
  {
    accessorKey: "expire",
    header: "Expire after",
  },
  {
    accessorKey: "burn",
    header: "Burn",
  },
  {
    accessorKey: "views",
    header: ({ column }) => {
      return (
        <div
        className="cursor-pointer hover:text-current"
          onClick={() => column.toggleSorting(column.getIsSorted() === "asc")}
        >
          Views
        </div>
      )
    },
  },
]
